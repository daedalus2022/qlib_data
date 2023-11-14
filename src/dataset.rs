use polars::frame::DataFrame;

///
/// 数据集
///
trait Dataset{
    ///
    /// Config被设计用来配置无法从数据中学习到的参数
    ///
    fn config(self, config: serde_json::Value);

    ///
    /// Setup the data. 设置数据。
    // We split the setup_data function for following situation:
    // - User have a Dataset object with learned status on disk. 用户在磁盘上有一个具有学习状态的Dataset对象
    // - User load the Dataset object from the disk. 用户从磁盘加载Dataset对象。
    // - User call `setup_data` to load new data. 用户调用' setup_data '来加载新数据。
    // - User prepare data for model based on previous status. 用户根据以前的状态为模型准备数据。
    ///
    ///
    fn setup_date(self);

    ///
    /// The type of dataset depends on the model. (It could be pd.DataFrame, pytorch.DataLoader, etc.) 数据集的类型取决于模型。(可能是pd。DataFrame pytorch。DataLoader等等)。
    // The parameters should specify the scope for the prepared data 参数应该指定准备数据的范围
    // The method should:
    // - process the data
    // - return the processed data
    ///
    fn prepare(self);
}

///
/// 序列化
///
trait Serializable{
    ///
    /// 持久化
    ///
    fn to_pickle(self, path: String)->anyhow::Result<bool>;

    ///
    /// 加载持久化数据
    ///
    fn load(self, path: String) -> anyhow::Result<DataFrame>;
}

///
/// 数据加载
///
trait DataLoader{
    fn load(self, instruments: String, start_end: StratEndRange) -> anyhow::Result<DataFrame>;
}

///
/// Dataset with Data(H)andler
/// User should try to put the data preprocessing functions into handler. 用户应该尝试将数据预处理函数放入处理程序中。
//  Only following data processing functions should be placed in Dataset: 数据集中只应放置以下数据处理函数:
// - The processing is related to specific model. 处理与具体的模型有关。
// - The processing is related to data split. 处理与数据分割有关。
///
///
struct DatasetH{
    pub segments: Segments,
    pub handler: DataHandler
}

impl Dataset for DatasetH{
    ///
    /// 解析配置
    ///
    fn config(self, config: serde_json::Value) {
        todo!()
    }

    fn setup_date(self) {
        // self.handler.setup_data();
        todo!()
    }

    fn prepare(self) {
        todo!()
    }
}

///
/// 数据处理
///
struct DataHandler{
    ///
    /// The stock list to retrieve. 基线股票集合
    ///
    pub instruments: String,

    ///
    /// start_time of the original data. 原始数据的开始时间
    ///
    pub start_time: String,

    ///
    /// end_time of the original data.  原始数据的结束时间
    ///
    pub end_time :String,

    ///
    /// data loader to load the data.
    ///
    pub data_loader : Box<dyn DataLoader>,

    ///
    /// initialize the original data in the constructor.
    ///
    pub  init_data: String,

    ///
    /// Return the original data instead of copy if possible.
    ///
    pub fetch_orig:bool,

}

impl Serializable for DataHandler{
    fn to_pickle(self, path: String)->anyhow::Result<bool> {
        todo!()
    }

    fn load(self, path: String) -> anyhow::Result<DataFrame> {
        todo!()
    }
}



///
/// 片段
/// 1)
/// ```json
/// 'segments': {
///    'train': ("2008-01-01", "2014-12-31"),
///    'valid': ("2017-01-01", "2020-08-01",),
///    'test': ("2015-01-01", "2016-12-31",),
///  }
/// ```
/// 2)
/// ```json
///  'segments': {
///    'insample': ("2008-01-01", "2014-12-31"),
///    'outsample': ("2017-01-01", "2020-08-01",),
///  }
/// ```
///
enum Segments{
    TVT(SegmentTVT),
    SIMPLE(SegmentsSimple)
}

///
/// 机器学习常用切片分组
///
struct SegmentTVT{
    pub train: StratEndRange,
    pub valid: StratEndRange,
    pub test: StratEndRange
}

///
/// 简单切片，包含输入输出范围
///
struct SegmentsSimple{
    pub insample: StratEndRange,
    pub outsample: StratEndRange
}

///
/// 开始结束范围
///
struct StratEndRange{
    pub start: String,
    pub end: String,
}



#[cfg(test)]
mod dataset_works{
    use super::{DatasetH, Segments, DataHandler};

    #[test]
    fn dataset_h_works(){
        let segments = Segments::TVT(super::SegmentTVT {
            train: super::StratEndRange {
                start: String::from("2008-01-01"),
                end: String::from("2017-12-31")
            },
            valid: super::StratEndRange {
                start: String::from("2008-01-01"),
                end: String::from("2017-12-31")
            },
            test: super::StratEndRange {
                start: String::from("2008-01-01"),
                end: String::from("2017-12-31")
            }
        });

        let handler: DataHandler = DataHandler{ instruments: todo!(), start_time: todo!(), end_time: todo!(), data_loader: todo!(), init_data: todo!(), fetch_orig: todo!() };

        let dataset = DatasetH{
            segments: segments,
            handler: handler,
        };




    }
}
