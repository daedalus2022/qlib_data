use anyhow::Result;
use clap::Parser;
use qlib_data::{
    command::{self, Opts, UpdateToday, UpdateDay},
    source_data,
};
use qshare::{
    sina::stock::{eastmoney::EastmoneySpotEmDataSource, sina::SinaIndexSpotDataSource},
    RealTimeData,
};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "qlib_data=debug");
    // 初始化日志
    tracing_subscriber::fmt::init();

    let opts: Opts = Opts::parse();

    match opts.updatecmd {
        command::UpdateCommand::UpdateToday(_args) => {
            // 更新当天股票数据
            update_today(None).await?;
        }

        command::UpdateCommand::UpdateDay(args) => {
            // 更新指定日期数据，前提是已经缓存过实时行情
            update_today(Some(args)).await?;
        },
    };

    Ok(())
}

///
/// 使用当天数据更新 source 数据
///
async fn update_today(args: Option<UpdateDay>) -> Result<()> {
    tracing::debug!("{:?} 更新当天实时行情数据", args);

    // 1. 使用东方财富数据源获取数据
    let sina_source = EastmoneySpotEmDataSource {};

    // 2. 股票实时数据行情
    let em_data = sina_source.real_time_data().await?;

    // 1. 使用sina数据源获取数据
    let sina_source = SinaIndexSpotDataSource {};

    // 3. 股指当天实时行情
    let spot_data = sina_source.real_time_data().await?;

    if let Some(args_opt) = args {
        if let Some(date) = args_opt.date{
            tracing::debug!("更新实时行情数据:{:?}", &em_data);
            source_data::update_today_data(
                Some(date.clone()),
                em_data.data,
                qlib_data::util::IoUtils::em_row_to_csv(),
            )
            .await?;
            tracing::debug!("更新实时行情数据:{:?}", &spot_data);
            source_data::update_today_data(
                Some(date.clone()),
                spot_data.data,
                qlib_data::util::IoUtils::spot_index_row_to_csv(),
            )
            .await?;
        }
    } else {
        source_data::update_today_data(
            None,
            em_data.data,
            qlib_data::util::IoUtils::em_row_to_csv(),
        )
        .await?;
        source_data::update_today_data(
            None,
            spot_data.data,
            qlib_data::util::IoUtils::spot_index_row_to_csv(),
        )
        .await?;
    }

    tracing::debug!("获取source目录下文件");

    tracing::debug!("遍历开始更新数据");

    Ok(())
}
