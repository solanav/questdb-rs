use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::error::SQLError;
use crate::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::types::{Schema, Atomicity};

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
    pub async fn exec<T: DeserializeOwned>(&self, query: &str, limit: Option<usize>, count: Option<bool>, nm: Option<bool>)
        -> Result<Vec<T>, crate::error::Error>
    {
        let mut url = format!("{}/exec?query={}", self.url, query);

        // Check all the optional arguments and add them to the URL
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

    /// The function `imp` streams tabular text data directly into a table. It supports CSV, TAB and
    /// Pipe (|) delimited inputs and optional headers. There are no restrictions on data size. Data
    /// type and structure is detected automatically and usually without additional configuration.
    /// However in some cases additional configuration can be provided to augment automatic
    /// detection results.
    pub async fn imp(&self, file_path: &'static str, schema: Option<Vec<(&'static str, Schema)>>, table_name: Option<&'static str>,
                     overwrite: Option<bool>, durable: Option<bool>, atomicity: Option<Atomicity>)
        -> Result<(), crate::error::Error>
    {
        let mut form = reqwest::multipart::Form::new();
        let mut url = format!("{}/imp?fmt=json", self.url);

        // Check all the optional arguments and add them to the URL
        if let Some(s) = schema {
            let mut data = String::new();

            for (i, &(name, schema)) in s.iter().enumerate() {
                if i == s.len() - 1 {
                    data += format!("{}={}", name, schema).as_str();
                } else {
                    data += format!("{}={}&", name, schema).as_str();
                }
            }

            form = form.part("schema", reqwest::multipart::Part::text(data));
        }
        if let Some(t) = table_name {
            url += format!("&name={}", t).as_str();
        }
        if let Some(o) = overwrite {
            url += format!("&overwrite={}", o).as_str();
        }
        if let Some(d) = durable {
            url += format!("&durable={}", d).as_str();
        }
        if let Some(a) = atomicity {
            url += format!("&atomicity={}", a).as_str();
        }

        println!("{}", url);

        // Read the file as bytes
        let filep = Path::new(file_path);
        let mut file = File::open(&filep)?;
        let mut file_bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut file_bytes)?;

        // Create a part with the file_name
        let file_name = match filep.file_name() {
            Some(name) => name.to_str().unwrap(),
            None => filep.to_str().unwrap(),
        };
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name);

        // Create the form with the file part
        form = form.part("data", part);

        // Make the POST request
        let res = self.client.post(url.as_str())
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;

        println!("[[[{:?}]]]", res);

        Ok(())
    }
}