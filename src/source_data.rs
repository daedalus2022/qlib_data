use csv::{ReaderBuilder, StringRecord};
use pbr::ProgressBar;
use polars::{
    export::num::ToPrimitive,
    frame::row::Row,
    lazy::dsl::{col, lit},
    prelude::{DataFrame, IntoLazy},
};
use std::fmt::Write as FmtWrite;
use std::{
    collections::HashMap,
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
                    if stock_datas
                        .iter()
                        .find(|s| {
                            s.get(
                                headers
                                    .iter()
                                    .position(|&x| x == const_vars::CSV_HEADER_DATE)
                                    .unwrap(),
                            )
                            .unwrap()
                            .to_string()
                                == current_date.clone()
                        })
                        .is_none()
                    {
                        let row_data_frame = data_frame
                            .clone()
                            .lazy()
                            .filter(col(const_vars::CSV_HEADER_SYMBOL).eq(lit(symbol_code)))
                            .collect()?;

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

fn row_to_csv(current_date: String, symbol: String, row: &Row, headers: Vec<&str>) -> String {
    let data = &row.0;

    let close = data[2].to_string();
    let open = data[10].to_string();
    let volume = data[5].to_string();
    let low = data[9].to_string();
    let high = data[8].to_string();
    let adjclose = data[11].to_string();
    let dividends = "0".to_string();
    let splits = "0".to_string();
    let symbol = symbol;
    let now_fmt = current_date;
    let date = now_fmt;

    let mut kv = HashMap::new();
    kv.insert("close", close);
    kv.insert("open", open);
    kv.insert("volume", volume);
    kv.insert("low", low);
    kv.insert("high", high);
    kv.insert("adjclose", adjclose);
    kv.insert("dividends", dividends);
    kv.insert("splits", splits);
    kv.insert("symbol", symbol);
    kv.insert("date", date);

    let mut csv_row = String::new();
    for h in &headers {
        if let Some(v) = kv.get(h) {
            csv_row
                .write_fmt(format_args!("{},", v.as_str()))
                .expect("格式化错误");
        } else {
            csv_row.write_fmt(format_args!(",")).expect("格式化错误");
        }
    }

    tracing::debug!("{:?}, {}", &headers, &csv_row);
    format!("{}\r\n", csv_row.as_str()[..csv_row.len() - 1].to_string()).to_string()
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
    use qshare::{sina::stock::SinaDataSource, RealTimeData};

    use crate::source_data::update_today_data;

    ///
    /// 更新股票指数当天数据
    ///
    // #[tokio::test]
    // pub async fn update_spot_today_data() -> anyhow::Result<()> {
    //     let sina = SinaDataSource {};
    //     let em_data = sina.real_time_spot_data().await?.data.unwrap();

    //     update_today_data(Some(em_data)).await?;

    //     Ok(())
    // }

    ///
    /// 更新股票当天数据
    ///
    #[tokio::test]
    pub async fn update_spot_em_today_data() -> anyhow::Result<()> {
        let sina = SinaDataSource {};
        let em_data = sina.real_time_spot_em_data().await?.data.unwrap();

        update_today_data(Some(em_data)).await?;

        Ok(())
    }
}
