//! The questdb crate allows for a simple way of connecting to a questdb instance.
//!
//! You can create a new connection using the QuestDB structure.

mod api;
mod error;

/// Object to connect to a questdb
pub use api::QuestDB;

/// Custom error
pub use error::Error;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: i32,
    ts: String,
    temp: f64,
    sensor_id: i32,
}

#[cfg(test)]
mod tests {
    use crate::api::QuestDB;
    use crate::TestData;
    use serde::Serialize;

    #[tokio::test]
    async fn it_works() {
        let connection = QuestDB::new("http://192.168.1.37:9000");
        let res = connection.exec::<TestData>("select * from readings", Some(5), None, None).await.unwrap();
        println!("{:#?}", res);
    }
}
