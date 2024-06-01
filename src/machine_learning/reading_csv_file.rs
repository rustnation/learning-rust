use std::fs::File;
extern crate polars;
use polars::prelude::*;

pub fn master(show: bool) -> Result<()>{
    if show {
        println!("--- Reading CSV File\n");

        let file = File::open("./files/machine_learning/heart_attack_prediction_dataset.csv")?;

        let df = CsvReader::new(file)
            .infer_schema(Some(100))
            .has_header(true)
            .finish()?;

        println!("DataFrame: {:?}", df.shape());
    }

    Ok(())
}