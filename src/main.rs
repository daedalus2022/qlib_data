use anyhow::Result;
use clap::Parser;
use qlib_data::{
    command::{self, Opts, UpdateToday},
    source_data,
};
use qshare::{sina::stock::SinaDataSource, RealTimeData};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "qlib_data=debug");
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 1. 使用sina数据源获取数据
    let sina_source = SinaDataSource {};

    let opts: Opts = Opts::parse();

    let result = match opts.updatecmd {
        command::UpdateCommand::UpdateToday(args) => update_today(sina_source, args).await?,

        command::UpdateCommand::UpdateDay(_args) => todo!(),
    };

    Ok(result)
}

///
/// 使用当天数据更新 source 数据
///
async fn update_today(sina_source: impl RealTimeData, args: UpdateToday) -> Result<()> {
    let em_data = sina_source.real_time_spot_em_data().await?;
    tracing::debug!("{:?} 获取实时行情数据:{:?}", args, &em_data);

    if let Some(date) = args.date {
        source_data::update_today_data(Some(date), em_data.data).await?;
    } else {
        source_data::update_today_data(None, em_data.data).await?;
    }

    tracing::debug!("获取source目录下文件");

    tracing::debug!("遍历开始更新数据");

    Ok(())
}
