use chrono::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Foo {
    Status {
        seq: u64,
        time: DateTime<Utc>,
    },
}

api!(Foo, "0.1.0");
