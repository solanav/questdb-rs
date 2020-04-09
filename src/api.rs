use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::error::SQLError;
use crate::Error;

pub struct QuestDB {
    client: Client,
    url: String,
}

impl QuestDB {
    /// Creates a new connection to questdb
    ///
    /// # Example
    /// ```
    /// use questdb::QuestDB;
    /// let connection = QuestDB::new("http://192.168.1.37:9000");
    /// ```
    pub fn new(url: &str) -> Self {
        QuestDB {
            client: Client::new(),
            url: String::from(url),
        }
    }

    /// Compiles and executes the SQL query supplied
    ///
    /// # Arguments
    /// * `query` - query text. It can be multi-line, but query separator, such as ; must not be
    /// included.
    /// * `limit` - This argument is used for paging. Limit can be either in format of X, Y where X is the
    /// lower limit and Y is the upper, or just Y. For example, limit=10,20 will return row
    /// numbers 10 thru to 20 inclusive. and limit=20 will return first 20 rows, which is
    /// equivalent to limit=0,20
    /// * `count` - Instructs /exec to count rows and return this value in message header. Default value
    /// is false. There is slight performance hit for requesting row count.
    /// * `nm` - Skips metadata section of the response when true. When metadata is known and client is
    /// paging this flag should typically be set to true to reduce response size. Default value
    /// is false and metadata is included in the response.
    ///
    /// # Example
    /// ```no-test
    /// use questdb::QuestDB;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct TestData {
    ///     id: i32,
    ///     ts: String,
    ///     temp: f64,
    ///     sensor_id: i32,
    /// }
    ///
    /// let connection = QuestDB::new("http://192.168.1.37:9000");
    /// let res = connection.exec::<TestData>("select * from readings", Some(5), None, None)
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn exec<T: DeserializeOwned>(&self, query: &str, limit: Option<u128>, count: Option<bool>, nm: Option<bool>)
        -> Result<Vec<T>, crate::error::Error>
    {
        let mut url = format!("{}/exec?query={}", self.url, query);

        if let Some(l) = limit {
            url += format!("&limit={}", l).as_str();
        }

        if let Some(c) = count {
            url += format!("&count={}", c).as_str();
        }

        if let Some(n) = nm {
            url += format!("&nm={}", n).as_str();
        }

        let res = self.client.get(url.as_str())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let deserialized = match res.get("dataset") {
            Some(d) => d,
            None => {
                // The SQL failed, return an error with the error data
                let e: SQLError = serde_json::from_value(res)?;
                return Err(Error::SQLError(e));
            },
        }.to_owned();

        let deserialized: Vec<T> = serde_json::from_value(deserialized)?;

        Ok(deserialized)
    }
}