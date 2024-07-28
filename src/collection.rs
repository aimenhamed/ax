use crate::request::{create_request, invoke_request, set_headers};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub name: String,
    pub url: String,
    pub method: String,
    pub headers: Vec<String>,
    pub data: Option<Value>,
    pub print: Vec<String>,
}

pub fn run_collection(collection_file_path: &String) -> Result<(), ureq::Error> {
    if let Ok(collection) = read_json_file::<Collection>(collection_file_path) {
        return invoke_collection(collection);
    }
    eprintln!("Cannot find collection file: {}", collection_file_path);
    Ok(())
}

pub fn run_collections(collection_file_path: &String) -> Result<(), ureq::Error> {
    if let Ok(collections) = read_json_file::<Vec<Collection>>(collection_file_path) {
        for collection in collections {
            invoke_collection(collection)?;
            println!("\n")
        }
        return Ok(());
    }
    eprintln!("Cannot find collection file: {}", collection_file_path);
    Ok(())
}

fn invoke_collection(collection: Collection) -> Result<(), ureq::Error> {
    let mut request = match create_request(collection.method, &collection.url) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let headers: Vec<&String> = collection.headers.iter().collect();
    let is_json_request = headers
        .iter()
        .any(|h| h.to_lowercase().trim() == "content-type:application/json");
    request = match set_headers(headers, request, is_json_request) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let data = match collection.data {
        Some(d) => Some(d.to_string()),
        None => None,
    };

    let response = invoke_request(request, data.as_ref())?;
    let name = collection.name;

    println!("\x1b[1mCollection:\x1b[0m {name}\n");
    pick_response_prints(response, collection.print)
}

fn read_json_file<T: DeserializeOwned>(file_path: &str) -> serde_json::Result<T> {
    let mut file = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let collection: T =
        serde_json::from_str(&contents).expect("Missing fields in given collection file");

    Ok(collection)
}

fn pick_response_prints(res: ureq::Response, prints: Vec<String>) -> Result<(), ureq::Error> {
    if prints.iter().any(|p| p == "status_code") {
        let code = res.status();
        println!("\x1b[1mStatus Code:\x1b[0m {code}");
    }

    if prints.iter().any(|p| p == "status_text") {
        let status = res.status_text();
        println!("\x1b[1mStatus Text:\x1b[0m {status}");
    }

    if prints.iter().any(|p| p == "headers") {
        let headers = res.headers_names().into_iter().collect::<Vec<_>>();
        let header_values: Vec<(String, String)> = headers
            .iter()
            .filter_map(|header| {
                res.header(header)
                    .map(|value| (header.clone(), value.to_string()))
            })
            .collect();
        for (header, value) in header_values {
            println!("\x1b[1m{header}:\x1b[0m {value}");
        }
    }

    if prints.iter().any(|p| p == "body") {
        let body = res.into_string()?;
        println!("\x1b[1mBody:\x1b[0m {body}");
    }

    Ok(())
}
