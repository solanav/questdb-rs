//! The questdb crate allows for a simple way of connecting to a questdb instance.
//!
//! You can create a new connection using the QuestDB structure.

mod api;
mod error;
mod types;

/// Object to connect to a questdb
pub use api::QuestDB;

/// Custom error
pub use error::Error;

#[cfg(test)]
mod tests {
    use crate::api::QuestDB;
    use crate::types::Atomicity;
    use serde::Deserialize;
    use std::fs::File;

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct TestData {
        id: i32,
        ts: String,
        temp: f64,
        sensor_id: i32,
    }

    #[tokio::test]
    async fn test_exec() {
        let connection = QuestDB::new("http://192.168.1.37:9000");
        let _res = match connection
            .exec::<TestData>("select * from readings", Some(5), None, None)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
    }

    #[tokio::test]
    async fn test_imp() {
        let connection = QuestDB::new("http://192.168.1.37:9000");
        let _res = match connection
            .imp(
                "./links.csv",
                "nu_table",
                Some(false),
                Some(true),
                Some(Atomicity::Strict),
            )
            .await
        {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
    }

    #[tokio::test]
    async fn test_exp() {
        let connection = QuestDB::new("http://192.168.1.37:9000");

        let mut output_file = File::create("output.csv").unwrap();
        let _res = match connection
            .exp("select * from nu_table", Some(5), &mut output_file)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
    }
}
