pub mod command;
pub mod const_vars;
pub mod normalize_data;
pub mod source_data;
pub mod util;
///
/// 数据源类型
///
pub enum Source {
    SINA,
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        if "sina" == value {
            return Source::SINA;
        }

        Source::SINA
    }
}
