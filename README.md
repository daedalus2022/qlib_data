# qlib_data

## 项目目标
1. 提供qlib回测所需股票日k历史数据2008年到2023年10月27日（每月更新一次）（qlib提供的数据比较旧，虽然可以使用qlib自带的data_collector更新，因为使用的yahoo数据源，对国内用户不太友好而且更新速度比较慢）
    1. 使用`cat *.zip.* > source.zip`命令合并zip文件[zip -s 50m source-20080101-20231201.zip  --out source-20080101-20231201-split.zip]
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


## libtorch 安装
1. 下载： https://download.pytorch.org/libtorch/cpu/libtorch-macos-2.1.0.zip
2. macos 配置环境变量：*注意三个地址都配置到libtorch的根目录，不要加lib和incloud*
export LIBTORCH_INCLUDE=/Users/tom/Downloads/libtorch
export LIBTORCH_LIB=/Users/tom/Downloads/libtorch
export LIBTORCH=/Users/tom/Downloads/libtorch
export DYLD_LIBRARY_PATH=/Users/tom/Downloads/libtorch/lib:$DYLD_LIBRARY_PATH
3. ubuntu22.04.3版本上使用 Tesla M40（NVIDIA-Linux-x86_64-535.129.03.run） cuda118版本，测试tch cuda版本通过（缺少包可以拷贝到debug下）
- `export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH`
- `cargo test --test tch_test`
