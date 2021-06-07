use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct ApiRes {
    ip: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let res = reqwest::blocking::get("https://api.myip.com")?.json::<ApiRes>()?;

    println!("{}", res.ip);

    Ok(())
}
