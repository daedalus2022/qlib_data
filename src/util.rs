use std::{fs, io::Error, path::Path};

use polars::export::chrono::Local;

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
}
