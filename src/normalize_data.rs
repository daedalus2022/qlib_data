use crate::const_vars::{self, CSV_HEADER_ADJCLOSE};
use async_std::fs::{self as sync_fs, DirEntry};
use async_std::stream::StreamExt;
use pbr::ProgressBar;
use polars::export::num::ToPrimitive;
use polars::lazy::dsl::{col, lit};
use polars::prelude::{
    AnyValue, CsvReader, CsvWriter, DataFrame, IntoLazy, SerReader, SerWriter, UniqueKeepStrategy,
};
use std::f64::NAN;
use std::fs;

use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::task;

///
/// 遍历source目录
///
pub async fn traverse_source_directory(dir: &str) -> anyhow::Result<()> {
    // 遍历文件夹
    let mut tasks = vec![];

    let dir_path = Path::new(&dir);

    if let Ok(source_dir) = fs::read_dir(dir_path) {
        let process_count = source_dir.count();
        let pb: Arc<Mutex<ProgressBar<std::io::Stdout>>> = Arc::new(Mutex::new(ProgressBar::new(
            process_count.to_u64().unwrap(),
        )));
        pb.lock().unwrap().format("╢▌▌░╟");

        // 异步遍历文件
        let mut entries = sync_fs::read_dir(dir_path).await?;
        while let Some(entry_res) = entries.next().await {
            let pbr = pb.clone();
            if let Ok(entry) = entry_res {
                let entry_clone = Arc::new(entry);
                // process(entry_clone.clone(), pbr).await?;
                tasks.push(task::spawn(process(entry_clone.clone(), pbr)));
            }
        }

        // 等待所有任务完成
        for task in tasks {
            let _ = task.await?;
        }

        pb.lock().unwrap().finish_println("处理完成");
    }

    Ok(())
}

///
/// csv 文件标准化
///
async fn process(
    path: Arc<DirEntry>,
    pb: Arc<Mutex<ProgressBar<std::io::Stdout>>>,
) -> anyhow::Result<()> {
    println!("update file :{:?}", path);
    if let Some(file) = path.path().to_str() {
        let mut df = CsvReader::from_path(file).unwrap().finish().unwrap();

        df = normalize(Some(df)).await?;

        tracing::debug!("normalize: {}", df);

        let mut file = std::fs::File::create(file.replace("source", "normalize")).unwrap();
        CsvWriter::new(&mut file).finish(&mut df).unwrap();

        pb.lock().unwrap().inc();

        return Ok(());
    }

    Ok(())
}

///
/// 生成 factor
/// ```py
///         if "adjclose" in df:
// df["factor"] = df["adjclose"] / df["close"]
// df["factor"] = df["factor"].fillna(method="ffill")
// else:
// df["factor"] = 1
// for _col in self.COLUMNS:
// if _col not in df.columns:
//     continue
// if _col == "volume":
//     df[_col] = df[_col] / df["factor"]
// else:
//     df[_col] = df[_col] * df["factor"]
///
/// ```
/// date,open,close,high,low,volume,money,change,adjclose,symbol,dividends,splits
/// date,open,high,volume,low,close,adjclose,dividends,splits,symbol
///
pub async fn adjusted_price(df: Option<DataFrame>) -> anyhow::Result<DataFrame> {
    if let Some(df) = df {
        let colum_names = df.get_column_names();
        let mut cols = vec![];
        for c in colum_names {
            cols.push(col(c));
        }

        if df.get_column_names().contains(&CSV_HEADER_ADJCLOSE) {
            cols.push((col("adjclose") / col("close")).alias("factor"));
            let df_manual = df.clone().lazy().select(cols).collect()?;

            let mut cols = vec![];
            let p_c = ["open", "close", "high", "low"];
            for c in df_manual.get_column_names() {
                if "volume" == c {
                    cols.push(col(c) / col("factor"));
                } else if p_c.contains(&c) {
                    cols.push(col(c) * col("factor"));
                } else {
                    cols.push(col(c));
                }
            }

            //["open", "close", "high", "low", "volume"]
            let df_manual = df_manual.clone().lazy().select(cols).collect()?;

            return Ok(df_manual);
        } else {
            let df_manual = df
                .clone()
                .lazy()
                .with_columns([col("factor").fill_null(lit(1))])
                .select(cols)
                .collect()?;
            return Ok(df_manual);
        }
    }
    Ok(DataFrame::empty())
}

///
/// df 标准化归一化处理
///
async fn normalize(df: Option<DataFrame>) -> anyhow::Result<DataFrame> {
    if let Some(df) = df {
        // 1. 删除数据框中重复的行，保留每行的第一个副本 df = df[~df.index.duplicated(keep="first")]
        let df = df
            .unique_stable(
                Some(&[String::from(const_vars::CSV_HEADER_DATE)]),
                UniqueKeepStrategy::Last,
            )
            .unwrap();
        // 2. 按时间字段排序
        let sort_df = df.sort([const_vars::CSV_HEADER_DATE], vec![false]).unwrap();

        tracing::debug!("删除数据框中重复的行，保留每行的第一个副本 df:{:?}", df);
        // TODO: 2.字符串数据类型和不保留默认 NaN 值
        // 3. 归一化/标准化处理
        // 4. manual adjust data: All fields (except symbol、adjclose、change) are standardized according to the close of the first day
        if let Some(first_close) = get_first_close(&sort_df).await {
            let df = adjusted_price(Some(sort_df)).await.ok().unwrap();
            tracing::debug!("adjusted_price df :{}", df);

            let mut cols = vec![];
            let p_c = ["open", "close", "high", "low", "money", "factor"];
            for c in df.get_column_names() {
                if "volume" == c {
                    cols.push(col(c) * lit(first_close));
                } else if p_c.contains(&c) {
                    cols.push(col(c) / lit(first_close));
                } else {
                    cols.push(col(c));
                }
            }

            let df_manual = df
                .clone()
                .lazy()
                .with_columns([
                    (col("dividends").fill_null(NAN)),
                    (col("splits").fill_null(NAN)),
                ])
                .select(cols)
                .collect()?;
            tracing::debug!("df_manual:{}", df_manual);

            return Ok(df_manual);
        }

        return Ok(DataFrame::empty());
    }

    Ok(DataFrame::empty())
}

///
/// 获取first close
///
pub async fn get_first_close(sort_df: &DataFrame) -> Option<f64> {
    let head_1_df = sort_df.head(Some(1));
    let dates = head_1_df.column(const_vars::CSV_HEADER_CLOSE).ok().unwrap();

    if let AnyValue::Float64(first_close) = dates.get(0).ok().unwrap() {
        return Some(first_close);
    }

    None
}

pub fn tch_normalization(tch_matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    // 计算均值向量
    let mean_vector = calculate_mean_vector(tch_matrix);
    // 计算标准差向量
    let std_vector = calculate_standard_deviation_vector(tch_matrix);
    // 归一化处理
    normalize_matrix(tch_matrix, &mean_vector, &std_vector)
}

fn calculate_mean_vector(tch_matrix: &[Vec<f64>]) -> Vec<f64> {
    let mut mean_vector = vec![];
    for row in tch_matrix {
        let mut sum = 0.0;
        for &value in row {
            sum += value;
        }
        mean_vector.push(sum / tch_matrix.len() as f64);
    }
    mean_vector
}
fn calculate_standard_deviation_vector(tch_matrix: &[Vec<f64>]) -> Vec<f64> {
    let mut std_vector = vec![];
    for row in tch_matrix {
        let mut variance = 0.0;
        let mean = calculate_mean(row);
        for &value in row {
            variance += (value - mean) * (value - mean);
        }
        std_vector.push(variance.sqrt());
    }
    std_vector
}

fn calculate_mean(numbers: &Vec<f64>) -> f64 {
    let sum = numbers.iter().sum::<f64>();
    let length = numbers.len() as f64;
    sum / length
}

fn normalize_matrix(
    tch_matrix: &[Vec<f64>],
    mean_vector: &[f64],
    std_vector: &[f64],
) -> Vec<Vec<f64>> {
    let mut normalized_tch_matrix = vec![Vec::new(); tch_matrix.len()];
    for (i, row) in tch_matrix.iter().enumerate() {
        let mut normalized_row = vec![0.0; row.len()];
        for (j, &value) in row.iter().enumerate() {
            normalized_row[j] = (value - mean_vector[i]) / std_vector[i];
        }
        normalized_tch_matrix.push(normalized_row);
    }
    normalized_tch_matrix
}

#[cfg(test)]
mod test {
    use polars::prelude::{CsvReader, SerReader};

    use crate::normalize_data::{
        adjusted_price, get_first_close, tch_normalization, traverse_source_directory,
    };
    use crate::util::Envs;

    use super::normalize;

    #[test]
    pub fn normalization_works() {
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        // 初始化日志
        tracing_subscriber::fmt::init();

        let tch_matrix: Vec<Vec<f64>> = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let normalized_tch_matrix = tch_normalization(&tch_matrix);
        for row in normalized_tch_matrix {
            tracing::debug!("row: {:?}", row);
        }
    }

    #[tokio::test]
    async fn adjusted_price_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        // 初始化日志
        tracing_subscriber::fmt::init();

        let path = "source/sz300841.csv";
        let df = CsvReader::from_path(path).unwrap().finish().unwrap();

        let df = adjusted_price(Some(df)).await?;
        tracing::debug!("df:{:?}", df);

        let path = "source/sh601500.csv";
        let df = CsvReader::from_path(path).unwrap().finish().unwrap();

        let df = adjusted_price(Some(df)).await?;
        tracing::debug!("df:{:?}", df);

        let path = "source/sh000300.csv";
        let df = CsvReader::from_path(path).unwrap().finish().unwrap();

        let df = adjusted_price(Some(df)).await?;
        tracing::debug!("df:{:?}", df);

        Ok(())
    }

    #[tokio::test]
    pub async fn normalize_format_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        // 初始化日志
        tracing_subscriber::fmt::init();

        let path = "source/sz300841.csv";
        let df = CsvReader::from_path(path).unwrap().finish().unwrap();

        tracing::debug!("df:{:?}", df);

        let df = normalize(Some(df)).await?;

        tracing::debug!("normalize df:{:?}", df);

        let path = "source/sh000300.csv";
        let df = CsvReader::from_path(path).unwrap().finish().unwrap();

        tracing::debug!("df:{:?}", df);

        let df = normalize(Some(df)).await?;

        tracing::debug!("normalize df:{:?}", df);

        Ok(())
    }
    #[tokio::test]
    pub async fn get_first_close_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        // 初始化日志
        tracing_subscriber::fmt::init();

        let path = "source/sh000300.csv";
        let df = CsvReader::from_path(path).unwrap().finish().unwrap();

        tracing::debug!("df:{:?}", df);

        let first_close = get_first_close(&df).await;

        assert_eq!(first_close.unwrap(), 5385.1_f64);

        Ok(())
    }

    #[tokio::test]
    pub async fn traverse_source_directory_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "qlib_data=debug");

        traverse_source_directory(&Envs::source_home()).await?;

        Ok(())
    }
}
