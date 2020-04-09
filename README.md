# QuestDB

Async connector for [QuestDB](https://www.questdb.io/), a high performance time-series database.

# Usage
```
[dependencies]
questdb = "0.1"
```

# Example
```rust
use questdb::QuestDB;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: i32,
    ts: String,
    temp: f64,
    sensor_id: i32,
}

#[tokio::main]
async fn main() {
    let connection = QuestDB::new("http://192.168.1.37:9000");

    let res = connection.exec::<TestData>(
        "select * from readings",
        Some(2),
        None,
        None
    ).await.unwrap();

    println!("{:#?}", res);
    
    /* Output:
    [
        TestData {
            id: 1,
            ts: "2019-10-17T00:00:00.000000Z",
            temp: 16.470730545675295,
            sensor_id: 295,
        },
        TestData {
            id: 2,
            ts: "2019-10-17T00:00:00.100000Z",
            temp: 19.75780877621018,
            sensor_id: 9835,
        },
    ]
    */

}

```