use async_std::fs::{self as sync_fs, DirEntry};
use async_std::stream::StreamExt;
use pbr::ProgressBar;
use polars::export::num::ToPrimitive;
use polars::prelude::{CsvReader, CsvWriter, SerReader, SerWriter, DataFrame, UniqueKeepStrategy};
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
        let mut file = std::fs::File::create(file.replace("source", "normalize")).unwrap();
        CsvWriter::new(&mut file).finish(&mut df).unwrap();

        pb.lock().unwrap().inc();

        return Ok(());
    }

    Ok(())
}

////
/// df 标准化归一化处理
///
async fn normalize(df: Option<DataFrame>) -> anyhow::Result<DataFrame>{
    if let Some(df) = df{
        // 1. 删除数据框中重复的行，保留每行的第一个副本 df = df[~df.index.duplicated(keep="first")]
        let df = df.unique_stable(Some(&vec![String::from("date")]),UniqueKeepStrategy::Last).unwrap();
        tracing::debug!("删除数据框中重复的行，保留每行的第一个副本 df:{:?}", df);
        // TODO: 2.字符串数据类型和不保留默认 NaN 值
        // 3. 归一化/标准化处理



        return Ok(DataFrame::empty());
    }

    Ok(DataFrame::empty())
}





#[cfg(test)]
mod test {
    use polars::prelude::{CsvReader, SerReader};

    use crate::normalize_data::traverse_source_directory;
    use crate::util::Envs;

    use super::normalize;

    #[tokio::test]
    pub async fn normalize_format_works()->anyhow::Result<()>{
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        // 初始化日志
        tracing_subscriber::fmt::init();

        let path = "source/sh603105.csv";
        let  df = CsvReader::from_path(path).unwrap().finish().unwrap();

        tracing::debug!("df:{:?}", df);

        normalize(Some(df)).await?;

        Ok(())
    }

    #[tokio::test]
    pub async fn traverse_source_directory_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        traverse_source_directory(&Envs::source_home()).await?;
        Ok(())
    }
}
