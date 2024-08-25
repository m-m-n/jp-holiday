use encoding_rs;
use redis::Commands;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error as StdError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let holidays = fetch_holidays().await?;
    save_redis(holidays)?;

    Ok(())
}

async fn fetch_holidays() -> Result<HashMap<String, String>, Box<dyn StdError>> {
    let url = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";

    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;

    let (decoded, _, _) = encoding_rs::SHIFT_JIS.decode(&body);
    let utf8_body = decoded.into_owned();

    let mut holidays = HashMap::new();
    let mut rdr = csv::ReaderBuilder::new().from_reader(utf8_body.as_bytes());

    for result in rdr.records() {
        let record = result?;
        let date = record.get(0).unwrap();
        let description = record.get(1).unwrap();

        let formatted_date = format_date(date)?;

        holidays.insert(formatted_date, description.to_string());
    }

    Ok(holidays)
}

fn format_date(date: &str) -> Result<String, Box<dyn StdError>> {
    let regex = Regex::new(r"(\d{4})[-/]?(\d{1,2})[-/]?(\d{1,2})")?;
    if let Some(captures) = regex.captures(date) {
        let year = captures.get(1).unwrap().as_str();
        let month = captures.get(2).unwrap().as_str();
        let day = captures.get(3).unwrap().as_str();

        return Ok(format!(
            "{:04}/{:02}/{:02}",
            year.parse::<u32>()?,
            month.parse::<u32>()?,
            day.parse::<u32>()?
        ));
    }

    Err("日付が不正です".into())
}

fn save_redis(holidays: HashMap<String, String>) -> Result<(), Box<dyn StdError>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    for (date, description) in holidays {
        println!("{}\t{}", date, description);
        con.set(date, description)?;
    }

    Ok(())
}
