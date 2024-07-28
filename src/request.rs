pub fn create_request(method: String, url: &String) -> Result<ureq::Request, String> {
    match method.as_str() {
        "GET" => Ok(ureq::get(url)),
        "POST" => Ok(ureq::post(url)),
        "PUT" => Ok(ureq::put(url)),
        "DELETE" => Ok(ureq::delete(url)),
        "HEAD" => Ok(ureq::head(url)),
        _ => Err(format!("Unsupported HTTP method: {}", method)),
    }
}

pub fn set_headers(
    headers: Vec<&String>,
    mut request: ureq::Request,
    is_json_request: bool,
) -> Result<ureq::Request, String> {
    for header in headers {
        let parts: Vec<&str> = header.split(':').collect();
        if parts.len() == 2 {
            let name = parts[0].trim();
            let value = parts[1].trim();
            if !request.has(name) {
                if name.eq_ignore_ascii_case("Content-Type") {
                    request = request.set(
                        name,
                        if is_json_request {
                            "application/json"
                        } else {
                            value
                        },
                    );
                } else {
                    request = request.set(name, value);
                }
            }
        } else {
            return Err(format!("Invalid header format: {}", header));
        }
    }
    if is_json_request && !request.has("Content-type") {
        request = request.set("Content-type", "application/json");
    }
    Ok(request)
}

pub fn invoke_request(
    request: ureq::Request,
    data: Option<&String>,
) -> Result<ureq::Response, ureq::Error> {
    if let Some(data) = data {
        match request.send_string(data) {
            Ok(response) | Err(ureq::Error::Status(_, response)) => Ok(response),
            Err(e) => Err(e),
        }
    } else {
        match request.call() {
            Ok(response) | Err(ureq::Error::Status(_, response)) => Ok(response),
            Err(e) => Err(e),
        }
    }
}

pub fn print_response(r: ureq::Response, include: bool) -> Result<(), ureq::Error> {
    if include {
        let ver = r.http_version().to_string();
        let code = r.status();
        let status = r.status_text().to_string();
        let headers = r.headers_names().into_iter().collect::<Vec<_>>();
        let header_values: Vec<(String, String)> = headers
            .iter()
            .filter_map(|header| {
                r.header(header)
                    .map(|value| (header.clone(), value.to_string()))
            })
            .collect();
        let body = r.into_string()?;
        println!("{ver} {code} {status}");
        for (header, value) in header_values {
            println!("\x1b[1m{header}:\x1b[0m {value}");
        }
        print!("\n{body}");
    } else {
        let body = r.into_string()?;
        print!("{body}");
    }
    Ok(())
}
