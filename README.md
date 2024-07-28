# ax

ax is a CLI HTTP client aiming for compatibility with `curl` with helpful additional params which are commonly used and written in Rust for speed and faster JSON parsing.

## Prerequisites

- cargo (for building the Rust program)

## Installation

- `git clone https://github.com/aimenhamed/ax`
- `./install.sh`

## Usage

- `ax --help` - Help command with usage help and all options
- `ax "http://example.com"` - A simple HTTP request
- `ax -i "http://example.com"` - A HTTP request with included protocol response headers in the output
- `ax -X POST "http://example.com -H "Content-type: application/json" -d '{"name":"Jim"}'` - POST request with JSON payload
- `ax -X POST "http://example.com -j -d '{"name":"Jim"}'` - POST request with JSON payload using header shorthand
- `ax -c examples/local.json` - You can define your request as a collection in a JSON file to be invoked
- `ax -l examples/list.json` - You can define your requests as list of collections in a JSON file to be invoked

## Collections

You can store HTTP requests that you commonly invoke into JSON files called collections which `ax` can read and invoke.
They are invoked via `ax -c <file>`, no other params can be passed as all config is read from the JSON file.
The JSON structure:

```json
{
  "name": "<collection name>",
  "url": "<url>",
  "method": "<http method>",
  "headers": ["<your headers>"],
  "print": ["status_code", "status_text", "headers", "body"] 
}
```
The valid values for print: `["status_code", "status_text", "headers", "body"]`.

You can store a list of requests in a single collection file, which structure follows similarly to a single collection, but as a list. Invoked via `ax -l <file>`.
The JSON structure:

```json
[
    {
      "name": "<collection name>",
      "url": "<url>",
      "method": "<http method>",
      "headers": ["<your headers>"],
      "print": ["status_code", "status_text", "headers", "body"] 
    }
]
```
