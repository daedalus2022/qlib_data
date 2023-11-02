use async_std::fs::{self as sync_fs, DirEntry};
use async_std::stream::StreamExt;
use pbr::ProgressBar;
use polars::export::num::ToPrimitive;
use polars::prelude::{CsvReader, CsvWriter, SerReader, SerWriter};
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

// async fn normalize(df: DataFrame) -> anyhow::Result<DataFrame>{
//     //1. 初始化

//     // 1. 删除数据框中重复的行，保留每行的第一个副本 df = df[~df.index.duplicated(keep="first")]
//     // 2.
//     Ok(DataFrame::empty())
// }

#[cfg(test)]
mod test {
    use crate::normalize_data::traverse_source_directory;
    use crate::util::Envs;

    #[tokio::test]
    pub async fn traverse_source_directory_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "qlib_data=debug");
        traverse_source_directory(&Envs::source_home()).await?;
        Ok(())
    }
}
