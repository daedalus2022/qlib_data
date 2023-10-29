# qlib_data

## 项目目标
1. 提供qlib回测所需股票日k历史数据2008年到2023年10月27日（每月更新一次）（qlib提供的数据比较旧，虽然可以使用qlib自带的data_collector更新，因为使用的yahoo数据源，对国内用户不太友好而且更新速度比较慢）
    1. 使用`cat *.zip.* > source.zip`命令合并zip文件
2. 基于原始日k数据提供每日数据更新，包括如下功能：
    1. 使用[qshare](https://crates.io/crates/qshare)下载当天数据并更新到source数据中
    2. 处理source数据生成normalize
    3. 使用normalize数据重新构建qlib_data
    4. 

## 使用说明
1. qshare
    1. 通过.env 配置缓存路径
2. 命了说明
    1. 更新当天数据`qlib_data update-today [2023-10-27]`