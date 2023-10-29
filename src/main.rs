use anyhow::Result;
use clap::Parser;
use qlib_data::{command::{Opts, self}, source_data};
use qshare::{sina::stock::SinaDataSource, RealTimeData};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 1. 使用sina数据源获取数据
    let sina_source = SinaDataSource {};

    let opts: Opts = Opts::parse();
    let result = match opts.updatecmd {
        command::UpdateCommand::UpdateToday=>update_today(sina_source).await?,
        command::UpdateCommand::UpdateDay(_args) => todo!(), };

    Ok(result)
}

///
/// 使用当天数据更新 source 数据
///
async fn update_today(sina_source: impl RealTimeData) -> Result<()> {
    let em_data = sina_source.real_time_spot_em_data().await?;
    tracing::debug!("获取实时行情数据:{:?}", &em_data);
    source_data::update_today_data(em_data.data).await?;
    tracing::debug!("获取source目录下文件");

    tracing::debug!("遍历开始更新数据");

    Ok(())
}

