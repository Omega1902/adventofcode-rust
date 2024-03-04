use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

use std::fs;
use std::io;
use std::path::Path;

const BASE_URL: &str = "https://adventofcode.com/";

fn download(year: u16, day: u8, client: &Client) -> Option<bool> {
    let filename = format!("./data/{year}/day{day:02}.txt");
    let filepath = Path::new(&filename);
    if filepath.exists() {
        //not necessary to download
        return Some(false);
    }
    let url = format!("{BASE_URL}{year}/day/{day}/input");
    let mut result = match client.get(&url).send() {
        Ok(r) => r,
        Err(e) => {
            println!("Error loading from {url}: {e:?}");
            return None;
        }
    };
    if !result.status().is_success() {
        println!("Error loading from {url}: Status {:?}", result.status());
        return None;
    }
    let mut file = match fs::File::create(filepath) {
        Err(e) => {
            println!("Error writing file {filename}: {e:?}");
            return None;
        }
        Ok(f) => f,
    };
    match io::copy(&mut result, &mut file) {
        Err(e) => {
            println!("Error writing file {filename}: {e:?}");
            None
        }
        _ => Some(true),
    }
}

fn download_wrapper(year: u16, day: u8, client: &Client) {
    match download(year, day, client) {
        Some(true) => println!("Successfully downloaded {year} {day}"),
        Some(false) => println!("Not necessary to download {year} {day}"),
        None => println!("Error on downloading {year} {day}"),
    }
}

pub fn download_all(day: Option<u8>) {
    let session_id = match fs::read_to_string(".session") {
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => println!(".session is not available!\n Please login to https://adventofcode.com with your browser. Then lookup the session cookie and save the value to the .session file. It should be a long hexadecimal value."),
               _ => println!("Error reading session file '.session': {e:?}"),
            }
            return;
        }
        Ok(s_id) => s_id,
    };
    let cookie = format!("session={}", session_id.trim());
    let mut headers = HeaderMap::new();
    let cookies_value = match HeaderValue::from_str(&cookie) {
        Err(e) => {
            println!("Unable to create header value: {e:?}");
            return;
        }
        Ok(val) => val,
    };
    headers.insert(COOKIE, cookies_value);
    let client = Client::builder().default_headers(headers).build().expect("unable to create client");

    match day {
        Some(i) => download_wrapper(2023, i, &client),
        None => (1..=25).map(|i| download_wrapper(2023, i, &client)).collect(),
    };
}
