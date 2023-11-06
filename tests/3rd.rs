#[cfg(test)]
mod test_3rd_works {
    use polars::prelude::DataFrame;
    use polars::prelude::*;

    #[tokio::test]
    async fn polars_duplicated()->anyhow::Result<()>{
        let df: DataFrame = df!("date" => &["2023-10-31", "2023-10-31", "2023-10-30"],
                        "open" => &["3581.6759", "3582.6759", "3571.6759"])?;

        let df = df.unique_stable(Some(&vec![String::from("date")]),UniqueKeepStrategy::Last).unwrap();

        println!("polars_duplicated:{:?}", df);

        assert!(df.iter().count() == 2);

        Ok(())
    }

    #[test]
    fn polars_normalized_works() {
        let df_customers = df! (

            "customer_id" => &[1, 2, 3],
            "name" => &["Alice", "Bob", "Charlie"],
        )
        .unwrap();

        println!("{}", &df_customers);

        let df2: DataFrame = df_customers.describe(None).unwrap();

        println!("{}", &df2);



        // // 打印原始 DataFrame
        // println!("原始 DataFrame：");
        // df.display();
        // // 归一化处理
        // let df_normalized = df.normalize();
        // // 打印归一化后的 DataFrame
        // println!("归一化后的 DataFrame：");
        // df_normalized.display();
        // // 标准化处理
        // let df_standardized = df.standardize();
        // // 打印标准化后的 DataFrame
        // println!("标准化后的 DataFrame：");
        // df_standardized.display();
    }

    // use num::{Num, Wrapping};

    // // fn standardize(data: &[f64]) -> Vec<f64> {
    // //     let min = data.iter().min().unwrap();
    // //     let max = data.max();

    // //     let range = max - min;
    // //     let mut standardized_data = Vec::new();

    // //     for &value in data {
    // //         let normalized_value = (value - min) / range;
    // //         standardized_data.push(normalized_value);
    // //     }

    // //     standardized_data
    // // }

    // #[test]
    // fn standardize_works(){
    //     let mut data = vec![
    //         10.0, 20.0, 30.0, 40.0, 50.0,
    //         5.0, 15.0, 25.0, 35.0, 45.0
    //     ];
    //     Wrapping(data);
    //     // let standardized_data = standardize(&data);
    //     // for &value in standardized_data {
    //     //     println!("Standardized value: {}", value);
    //     // }
    // }

    // use itertools::zip;
    // use lightgbm::{Booster, Dataset};
    // use serde_json::json;

    // fn load_file(file_path: &str) -> (Vec<Vec<f64>>, Vec<f32>) {
    //     let rdr = csv::ReaderBuilder::new()
    //         .has_headers(false)
    //         .delimiter(b'\t')
    //         .from_path(file_path);
    //     let mut labels: Vec<f32> = Vec::new();
    //     let mut features: Vec<Vec<f64>> = Vec::new();
    //     for result in rdr.unwrap().records() {
    //         let record = result.unwrap();
    //         let label = record[0].parse::<f32>().unwrap();
    //         let feature: Vec<f64> = record
    //             .iter()
    //             .map(|x| x.parse::<f64>().unwrap())
    //             .collect::<Vec<f64>>()[1..]
    //             .to_vec();
    //         labels.push(label);
    //         features.push(feature);
    //     }
    //     (features, labels)
    // }

    // #[test]
    // fn lightgbm_works() -> std::io::Result<()> {
    //     let (train_features, train_labels) =
    //         load_file("../../lightgbm-sys/lightgbm/examples/regression/regression.train");
    //     let (test_features, test_labels) =
    //         load_file("../../lightgbm-sys/lightgbm/examples/regression/regression.test");
    //     let train_dataset = Dataset::from_mat(train_features, train_labels).unwrap();

    //     let params = json! {
    //         {
    //             "num_iterations": 100,
    //             "objective": "regression",
    //             "metric": "l2"
    //         }
    //     };

    //     let booster = Booster::train(train_dataset, &params).unwrap();
    //     let result = booster.predict(test_features).unwrap();

    //     let mut tp = 0;
    //     for (label, pred) in zip(&test_labels, &result[0]) {
    //         if (*label == 1_f32 && *pred > 0.5_f64) || (*label == 0_f32 && *pred <= 0.5_f64) {
    //             tp += 1;
    //         }
    //         println!("{}, {}", label, pred)
    //     }
    //     println!("{} / {}", &tp, result[0].len());
    //     Ok(())
    // }
}
