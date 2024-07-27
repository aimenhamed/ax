# ax

ax is a CLI HTTP client aiming for compatibility with `curl` with helpful additionals which are commonly used and written in Rust for speed and faster JSON parsing.

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
