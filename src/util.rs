use std::{fmt::Write, fs, io::Error, path::Path};

use polars::{
    export::{
        ahash::{HashMap, HashMapExt},
        chrono::Local,
    },
    frame::row::Row,
};

use crate::const_vars;

///
/// 环境工具
///
pub struct Envs;
impl Envs {
    ///
    /// 本地缓存目录
    ///
    pub fn cache_temp_home() -> String {
        dotenvy::var(const_vars::CACHE_TEMP_HOME).unwrap()
    }

    ///
    /// source目录
    ///
    pub fn source_home() -> String {
        dotenvy::var(const_vars::SOURCE_DATA_HOME).unwrap()
    }
}

pub struct DateUtils;

impl DateUtils {
    ///
    /// 年-月-日格式的日期
    ///
    pub fn now_fmt_ymd() -> String {
        let now = Local::now();
        now.format("%Y-%m-%d").to_string()
    }
}

pub struct IoUtils;

impl IoUtils {
    ///
    /// mkdir -p
    ///
    pub fn create_dir_recursive(path: &Path) -> Result<(), Error> {
        if path.exists() {
            Ok(())
        } else {
            match path.parent() {
                Some(parent) => IoUtils::create_dir_recursive(parent),
                None => fs::create_dir(path),
            }
        }
    }

    pub fn em_row_to_csv(
    ) -> fn(current_date: String, symbol: String, row: &Row, headers: Vec<&str>) -> String {
        |current_date: String, symbol: String, row: &Row, headers: Vec<&str>| -> String {
            let data = &row.0;

            let close = data[2].to_string();
            let open = data[10].to_string();
            let volume = data[5].to_string();
            let low = data[9].to_string();
            let high = data[8].to_string();
            let adjclose = data[11].to_string();
            let dividends = "0".to_string();
            let splits = "0".to_string();
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
            format!("{}\r\n", &csv_row.as_str()[..csv_row.len() - 1].to_string()).to_string()
        }
    }

    ///
    /// source->symbol,代码,名称,最新价,涨跌幅,涨跌额,成交量,成交额,买,最高,最低,今开,昨收,卖,时间
    /// target->date,open,close,high,low,volume,money,change,adjclose,symbol
    ///
    pub fn spot_index_row_to_csv(
    ) -> fn(current_date: String, symbol: String, row: &Row, headers: Vec<&str>) -> String {
        |current_date: String, symbol: String, row: &Row, headers: Vec<&str>| -> String {
            let data = &row.0;

            let close = data[3].to_string();
            let open = data[11].to_string();
            let volume = data[6].to_string();
            let money = data[7].to_string();
            let low = data[10].to_string();
            let high = data[9].to_string();
            let change = data[4].to_string();
            let adjclose = data[12].to_string();
            let now_fmt = current_date;
            let date = now_fmt;

            let mut kv = HashMap::new();

            kv.insert("date", date);
            kv.insert("open", open);
            kv.insert("close", close);
            kv.insert("high", high);
            kv.insert("low", low);
            kv.insert("volume", volume);
            kv.insert("money", money);
            kv.insert("change", change);
            kv.insert("adjclose", adjclose);
            kv.insert("symbol", symbol);

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
            format!("{}\r\n", &csv_row.as_str()[..csv_row.len() - 1].to_string()).to_string()
        }
    }
}
