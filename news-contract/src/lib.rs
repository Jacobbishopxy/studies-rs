#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct News {
    pub id: uuid::Uuid,
    pub desc: String,
    pub url: String,
}

impl Display for News {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.id, self.desc, self.url)
    }
}

#[cfg(test)]
mod news_contract_tests {
    use super::*;

    #[test]
    fn test_display() {
        let n = News {
            id: uuid::Uuid::new_v4(),
            desc: "google".to_string(),
            url: "google.com".to_string(),
        };

        println!("{}", n);
    }
}
