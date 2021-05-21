#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Queryable, PartialEq)]
pub struct News {
    pub id: String,
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
            id: "1".to_string(),
            desc: "google".to_string(),
            url: "google.com".to_string(),
        };

        println!("{}", n);
    }
}
