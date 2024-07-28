use clap::{Arg, ArgAction, Command};
mod collection;
use collection::run_collection;
mod request;
use request::{create_request, invoke_request, print_response, set_headers};

fn cli() -> Command {
    Command::new("ax")
        .version("0.1.0")
        .author("ax")
        .about("CLI HTTP client")
        .arg(
            Arg::new("url")
                .help("The URL to send the request to")
                .required_unless_present("collection")
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
        .arg(
            Arg::new("collection")
                .short('c')
                .long("collection")
                .help("Collection to run HTTP request on")
                .action(ArgAction::Set)
                .required_unless_present("url"),
        )
}

fn main() -> Result<(), ureq::Error> {
    let matches = cli().get_matches();

    let collection = matches.get_one::<String>("collection");
    if let Some(collection_file_path) = collection {
        return run_collection(collection_file_path);
    }

    let method = matches.get_one::<String>("method").unwrap().to_uppercase();
    let url = matches.get_one::<String>("url").unwrap();
    let headers: Vec<&String> = matches
        .get_many::<String>("header")
        .unwrap_or_default()
        .collect();
    let data = matches.get_one::<String>("data");
    let is_json_request = matches.get_flag("json-request");
    let include = matches.get_flag("include");

    let mut request = match create_request(method, url) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    request = match set_headers(headers, request, is_json_request) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let response = invoke_request(request, data)?;
    print_response(response, include)
}
