extern crate csv;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader},
    path::Path,
};

use crate::getters;
use crate::news;
use crate::types;

pub fn writerecs(
    file_name: String,
    header: &[&str],
    records: Vec<csv::StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_name.to_string())?;
    wtr.write_record(header);
    for r in records.iter() {
        wtr.write_record(r);
    }
    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn currencies() -> Result<(), reqwest::Error> {
    for s in CURRENCY_SYMBOLS.iter() {
        if let Ok(curs) = getters::get_currency(s.to_string()) {
            let write_fn = format!("./data/USD{}.csv", s.to_string());
            if let Ok(recs) = types::Intraday::to_records(&curs[0]) {
                writerecs(
                    write_fn,
                    &["date_time", &curs[0].ticker.to_string(), "volume"],
                    recs,
                );
            }
        }
    }
    Ok(())
}

pub fn sp500(start: String, write_header: bool) -> Result<(), csv::Error> {
    let symbs = read_tickers("./data/sp500tickers.txt");
    let index = symbs
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &symbs[index..symbs.len()];

    let headlines_fn = "./data/sp500_headlines.csv".to_string();
    let metadata_fn = "./data/sp500.csv".to_string();
    let mut meta_wtr = csv::Writer::from_path(&metadata_fn)?;
    let mut lines_wtr = csv::Writer::from_path(&headlines_fn)?;
    meta_wtr.write_record(&STOCK_HEADER);
    lines_wtr.write_record(&HEADLINES_HEADER);
    for s in todo_symbs.iter() {
        if let Ok(c) = getters::get_datastrip(s.to_string()) {
            if let Ok(headlines) = types::Root::to_headlines(&c[0]) {
                for r in headlines.iter() {
                    lines_wtr.write_record(r);
                }
            }
            let metadata_record = types::Root::to_record(&c[0]);
            meta_wtr.write_record(&metadata_record);
        }
    }
    meta_wtr.flush();
    lines_wtr.flush();
    Ok(())
}

pub fn news() -> Result<(), csv::Error> {
    let write_fn = "./data/news.csv";
    let mut wtr = csv::Writer::from_path(&write_fn)?;
    wtr.write_record(&NEWS_HEADER);
    for s in NEWS_SYMBOLS.iter() {
        if let Ok(news_vec) = getters::get_news(s.to_string()) {
            if let Ok(recs) = news::NewsVec::to_records(&news_vec) {
                for r in recs.iter() {
                    wtr.write_record(r);
                }
            }
        }
    }
    Ok(())
}

pub fn prices(start: String) -> Result<(), reqwest::Error> {
    let symbs = read_tickers("./data/sp500tickers.txt");
    let index = symbs
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &symbs[index..symbs.len()];
    for s in todo_symbs.iter() {
        if let Ok(hist) = getters::get_history(format!("{}%3AUS", s.to_string())) {
            if let Ok(recs) = types::Intraday::to_records(&hist[0]) {
                let write_fn = format!("./data/{}.csv", s.to_string());
                let price_col = format!("{}_price", &s.to_string());
                let vol_col = format!("{}_volume", &s.to_string());
                writerecs(write_fn, &["date_time", &price_col, &vol_col], recs);
            }
        }
    }
    Ok(())
}

pub fn commodities_prices(start: String) -> Result<(), reqwest::Error> {
    let index = COMMODITIES_SYMBOLS
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &COMMODITIES_SYMBOLS[index..COMMODITIES_SYMBOLS.len()];
    for s in todo_symbs.iter() {
        if let Ok(hist) = getters::get_history(format!("{}%3ACOM", s.to_string())) {
            if let Ok(recs) = types::Intraday::to_records(&hist[0]) {
                let write_fn = format!("./data/{}.csv", s.to_string());
                let price_col = format!("{}_price", &s.to_string());
                let vol_col = format!("{}_volume", &s.to_string());
                writerecs(write_fn, &["date_time", &price_col, &vol_col], recs);
            }
        }
    }
    Ok(())
}

pub const STOCK_HEADER: [&'static str; 15] = [
    "id",
    "short_name",
    "market_cap",
    "co_phone",
    "last_update",
    "average_volume30_day",
    "price",
    "open_price",
    "high_price",
    "low_price",
    "low_price52_week",
    "high_price52_week",
    "number_of_employees",
    "price_earnings_ratio",
    "shares_outstanding",
];

pub const CURRENCY_SYMBOLS: [&'static str; 35] = [
    "EUR", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN", "BRL", "CLP", "COP", "PEN", "CRC",
    "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF", "RUB", "TRY", "ILS", "KES", "ZAR",
    "MAD", "NZD", "PHP", "SGD", "IDR", "CNY", "INR", "MYR", "THB",
];

pub const NEWS_SYMBOLS: [&'static str; 5] = [
    "GOVERNMENT_BOND",
    "COMMODITY",
    "COMMON_STOCK",
    "CURRENCY",
    "BLOOMBERG_BARCLAYS_INDEX",
];
pub const COMMODITIES_SYMBOLS: [&'static str; 37] = [
    "CO1", "CL1", "XB1", "NG1", "HO1", "GC1", "SI1", "HG1", "C%201", "W%201", "CC1", "CT1", "LC1",
    "QS1", "JX1", "MO1", "JG1", "LMCADSO3", "LMAHDSO3", "LMZSDSO3", "LMSNDSO3", "O%201", "RR1",
    "S%201", "SM1", "BO1", "RS1", "KC1", "SB1", "JO1", "CT1", "OL1", "LB1", "JN1", "DL1", "FC1",
    "LH1",
];

pub const NEWS_HEADER: [&'static str; 3] = ["url", "headline", "date_time"];

pub const HEADLINES_HEADER: [&'static str; 4] = ["id", "url", "headline", "lastmod"];