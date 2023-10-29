///
/// 命令定义
///
use clap::Parser;

// 定义 qlib_data 的 CLI 的主入口，它包含若干个子命令

/// qlib_data 用于更新qlib相关数据
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Daedalus2022 <daedalus2022@163.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub updatecmd: UpdateCommand,
}

// 更新命令方法，目前支持 当天数据更新
#[derive(Parser, Debug)]
pub enum UpdateCommand {
    UpdateToday,
    UpdateDay(UpdateDay)
}

// update day 子命令

/// 更新当天股票数据
#[derive(Parser, Debug)]
pub struct UpdateDay {
    /// 使用数据源,默认sina
    source: Option<String>,
}

#[cfg(test)]
mod tests_command {
    #[test]
    fn test_opt() {
        
    }
}
