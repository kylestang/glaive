# Glaive

*Cut your http requests to pieces*

Glaive is an API reverse engineering tool. It makes an initial http request and then removes properties one by one to determine the smallest set of properties needed to return the same response.

The properties Glaive currently tests are:
- headers
- cookies
- query parameters
- request body

Requests are defined using a subset of the `cURL` CLI flags. Most of the time you should be able to take a `curl ...` command, replace it with `glaive ...`, and it will just work. Many apps also export requests as curl such as web browsers, postman, mitmproxy, etc.

## Installation

```bash
cargo install glaive
```

## Usage

```
Usage: glaive [OPTIONS] <URL>

Arguments:
  <URL>

Options:
  -X, --request <REQUEST>    [default: GET] [possible values: GET, POST, PUT, DELETE, HEAD, OPTIONS, CONNECT, PATCH, TRACE]
  -H, --header <HEADER>      key-value pair separated by a colon (:)
      --data-raw <RAW_DATA>  raw request body
      --compressed           this doesn't do anything, but is added for compatibility
  -h, --help                 Print help
```

Glaive does briefly spam the server with requests, please use responsibly.
