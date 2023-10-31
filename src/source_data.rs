use csv::{ReaderBuilder, StringRecord};
use pbr::ProgressBar;
use polars::{
    export::num::ToPrimitive,
    frame::row::Row,
    lazy::dsl::{col, lit},
    prelude::{DataFrame, IntoLazy},
};
use std::{
    fs::{self, DirEntry},
    io::Write,
    path::Path,
};

use crate::{const_vars, util::DateUtils};

///
/// 使用股票/股指数据 更新本地缓存
///
pub async fn update_today_data(
    date: Option<String>,
    data_frame_opt: Option<DataFrame>,
    row_to_csv: fn(current_date: String, symbol: String, row: &Row, headers: Vec<&str>) -> String,
) -> anyhow::Result<()> {
    match data_frame_opt {
        Some(data_frame) => {
            // 1. 配置数据源目录
            let data_source_path = format!("{}", "source").to_string();

            let mut current_date = DateUtils::now_fmt_ymd();

            if let Some(d) = date {
                current_date = d;
            }

            tracing::debug!("update date:{}", current_date);

            let dir_path = Path::new(&data_source_path);
            let entries = fs::read_dir(dir_path).expect("无法读取目录");
            let process_count = entries.count();
            let mut pb = ProgressBar::new(process_count.to_u64().unwrap());
            pb.format("╢▌▌░╟");

            let entries = fs::read_dir(dir_path).expect("无法读取目录");
            for entry in entries {
                let path = entry.expect("无法获取路径");
                tracing::debug!("update file :{:?}", &path);

                // 只处理.csv文件
                if !path
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .ends_with(".csv")
                {
                    continue;
                }

                let symbol = String::from(&path.file_name().to_str().unwrap()[..8]);
                let symbol_code = String::from(&path.file_name().to_str().unwrap()[2..8]);

                // 4. 读取缓存文件到vec
                if let Ok(stock_datas) = load_csv(&path.path()) {
                    // 表头
                    let headers: Vec<&str> = stock_datas.first().unwrap().iter().collect();
                    // 数据是否存在
                    let target_row_opt = stock_datas.iter().find(|s| {
                        s.get(
                            headers
                                .iter()
                                .position(|&x| x == const_vars::CSV_HEADER_DATE)
                                .unwrap(),
                        )
                        .unwrap()
                            == current_date.clone()
                    });

                    if target_row_opt.is_none() {
                        let row_data_frame = data_frame
                            .clone()
                            .lazy()
                            .filter(
                                col(const_vars::CSV_HEADER_SYMBOL)
                                    .eq(lit(symbol_code))
                                    .or(col(const_vars::CSV_HEADER_SYMBOL).eq(lit(symbol.clone()))),
                            )
                            .collect()?;
                        tracing::debug!("更新:{:?} 日数据:{:?}", current_date, row_data_frame);
                        if let Ok(row) = row_data_frame.get_row(0) {
                            let csv_row = row_to_csv(current_date.clone(), symbol, &row, headers);
                            tracing::debug!("append:{:?} ,{}, to:{:?}", &row, csv_row, path);
                            append_to_csv(&path, csv_row);
                        }
                    }
                } else {
                    tracing::warn!("csv 文件加载失败");
                }

                pb.inc();
            }
            pb.finish_print("done");
        }
        None => {
            tracing::info!("None data frame");
        }
    }

    Ok(())
}

///
/// vec data to csv
///
fn append_to_csv(path: &DirEntry, row: String) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path.path())
        .unwrap();

    // 将内容写入文件
    file.write_all(row.as_bytes()).expect("写入文件错误");
}

pub fn load_csv(path: &Path) -> anyhow::Result<Vec<StringRecord>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
    if let Ok(headers) = rdr.headers() {
        let mut sds: Vec<StringRecord> = vec![headers.clone()];
        for result in rdr.records() {
            let record = result?;
            sds.push(record);
        }
        return Ok(sds);
    } else {
        tracing::warn!("get hearder error:{:?}", path);
    }

    Ok(vec![])
}

#[cfg(test)]
mod test {
    use qshare::{
        sina::stock::{eastmoney::EastmoneySpotEmDataSource, sina::SinaIndexSpotDataSource},
        RealTimeData,
    };

    use crate::source_data::update_today_data;

    ///
    /// 更新股票指数当天数据
    ///
    #[tokio::test]
    pub async fn update_spot_today_data() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        // 初始化日志
        tracing_subscriber::fmt::init();

        let sina = SinaIndexSpotDataSource {};
        let em_data = sina.real_time_data().await?.data.unwrap();

        update_today_data(
            None,
            Some(em_data),
            crate::util::IoUtils::spot_index_row_to_csv(),
        )
        .await?;

        Ok(())
    }

    ///
    /// 更新股票当天数据
    ///
    #[tokio::test]
    pub async fn update_spot_em_today_data() -> anyhow::Result<()> {
        let source = EastmoneySpotEmDataSource {};
        let em_data = source.real_time_data().await?.data.unwrap();

        update_today_data(None, Some(em_data), crate::util::IoUtils::em_row_to_csv()).await?;

        Ok(())
    }
}
