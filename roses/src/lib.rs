extern crate chrono;
extern crate csv;
use chrono::Utc;
use regex::Regex;
use std::{
    //error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    thread,
    time::Duration,
};

pub const DELAY: std::time::Duration = Duration::from_millis(10);
pub const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36";

#[tokio::main]
pub async fn simple_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT.to_string())
        .build()?;
    let res = client.get(&url).send().await?;
    thread::sleep(DELAY);
    let body = res.text().await?;
    // println!("{}: {:#?}", url, body);
    println!("{}", url);
    Ok(body)
}

// simple fns arent useful, get 'cant start runtime from within runtime'
#[tokio::main]
pub async fn simple_json(url: String) -> Result<::serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT.to_string())
        .build()?;

    client
        .get(&url)
        .send()
        .await?
        .json::<::serde_json::Value>() // CHANGE TYPE
        .await
}

// change to take in File?
pub fn write_csv(
    filepath: &Path,
    data: Vec<Vec<String>>,
    header: &[&str],
) -> Result<(), csv::Error> {
    let mut wtr =
        csv::Writer::from_path(filepath).expect(format!("whtf csv {:?}", filepath).as_ref());
    println!("writing {} rows to {:?}", data.len(), filepath);
    wtr.write_record(header.clone())?;
    wtr.flush()?;
    let len = header.len();
    for row in data.iter() {
        assert_eq!(len, row.len()); // perf hit?
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn to_csv(
    file: File,
    data: Vec<Vec<String>>,
    header: Option<&[&str]>,
) -> Result<(), csv::Error> {
    // decide beforehand whether to append or not

    let mut wtr = csv::Writer::from_writer(file);
    if let Some(h) = header {
        wtr.write_record(h)?;
    }

    for row in data.iter() {
        wtr.write_record(row)?;
    }

    wtr.flush()?;
    //println!("wrote {} rows to somewhere (TODO File->Path)", data.len());
    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn simppath(s: String, sfx: String) -> String {
    //sfx enum x, f, us
    return format!(
        "../data/{}_{}_{}.csv",
        s.to_string(),
        sfx.to_string(),
        Utc::now().to_rfc3339(),
    );
}
pub fn yf_symb_from_url(url: String) -> Option<String> {
    //example
    let re = Regex::new(r"/chart/(?P<symb>.+).*\?").unwrap();
    if let Some(caps) = re.captures(&url) {
        return Some(caps.name("symb").unwrap().as_str().to_string());
    }
    return None;
}

pub fn symb_from_ndaq_url(url: String) -> Option<String> {
    //example
    let re = Regex::new(r"/quote/(?P<symb>.+).*/info").unwrap();
    if let Some(caps) = re.captures(&url) {
        return Some(caps.name("symb").unwrap().as_str().to_string());
    }
    return None;
}
