use clap::{Arg, ArgAction, Command};

fn cli() -> Command {
    Command::new("ax")
        .version("0.1.0")
        .author("ax")
        .about("CLI HTTP client")
        .arg(
            Arg::new("url")
                .help("The URL to send the request to")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("method")
                .short('X')
                .long("method")
                .help("HTTP method to use")
                .action(ArgAction::Set)
                .default_value("GET"),
        )
        .arg(
            Arg::new("data")
                .short('d')
                .long("data")
                .help("Payload to send to request")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("header")
                .short('H')
                .long("header")
                .help("HTTP headers to set")
                .action(ArgAction::Append)
                .num_args(1),
        )
        .arg(
            Arg::new("json-request")
                .short('j')
                .long("json-request")
                .help("Append json header, takes presedence over any other 'Content-type' sets")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("include")
                .short('i')
                .long("include")
                .help("Include protocol response headers in the output")
                .action(ArgAction::SetTrue),
        )
}

fn main() -> Result<(), ureq::Error> {
    let matches = cli().get_matches();

    let method = matches.get_one::<String>("method").unwrap().to_uppercase();
    let url = matches.get_one::<String>("url").unwrap();
    let headers: Vec<&String> = matches
        .get_many::<String>("header")
        .unwrap_or_default()
        .collect();
    let data = matches.get_one::<String>("data");
    let is_json_request = matches.get_flag("json-request");
    let include = matches.get_flag("include");

    let mut request = match method.as_str() {
        "GET" => ureq::get(url),
        "POST" => ureq::post(url),
        "PUT" => ureq::put(url),
        "DELETE" => ureq::delete(url),
        "HEAD" => ureq::head(url),
        _ => {
            eprintln!("Unsupported HTTP method: {}", method);
            return Ok(());
        }
    };

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
            eprintln!("Invalid header format: {}", header);
        }
    }
    if is_json_request && !request.has("Content-type") {
        request = request.set("Content-type", "application/json");
    }

    if let Some(data) = data {
        match request.send_string(data) {
            Ok(response) | Err(ureq::Error::Status(_, response)) => {
                print_response(response, include)
            }
            Err(e) => Err(e),
        }
    } else {
        match request.call() {
            Ok(response) | Err(ureq::Error::Status(_, response)) => {
                print_response(response, include)
            }
            Err(e) => Err(e),
        }
    }
}

fn print_response(r: ureq::Response, include: bool) -> Result<(), ureq::Error> {
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
