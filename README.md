# Glaive

*Cut your http requests to pieces*

Glaive is an API reverse engineering tool. It makes an initial http request and then tests every combination of the request properties to find the minimum set of properties needed to return the same response.

The properties Glaive currently tests are:
- headers
- cookies
- query parameters
- request body

Requests are defined using a subset of the `cURL` CLI flags. Most of the time you should be able to take a `curl ...` command, replace it with `glaive ...`, and it will just work. Many apps also export requests as curl such as web browsers, postman, mitmproxy, etc.

## Usage

```
Usage: glaive [OPTIONS] <URL>

Arguments:
  <URL>

Options:
  -c, --concurrency <CONCURRENCY>  [default: 10]
  -X, --request <REQUEST>          [default: GET] [possible values: GET, POST, PUT, DELETE, HEAD, OPTIONS, CONNECT, PATCH, TRACE]
  -H, --header <HEADER>            key-value pair separated by a colon (:)
      --data-raw <RAW_DATA>        raw request body
  -h, --help                       Print help
```

By default Glaive runs 10 concurrent requests at a time to prevent overloading the destination server. You can set a different limit with the `-c` flag. Glaive sends up to `2^(number of properties)` requests, please be careful not to DOS others.
