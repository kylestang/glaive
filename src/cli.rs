use clap::{Parser, ValueEnum};
use reqwest::{Method, Url};
use std::fmt::Display;

use crate::RequestProperty;

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, ValueEnum, Debug)]
#[value(rename_all = "UPPER")]
enum Methods {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
}

impl Display for Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser)]
struct Args {
    url: Url,

    // Curl options
    #[arg(short = 'X', long, default_value_t = Methods::GET)]
    request: Methods,

    #[arg(short = 'H', long, value_parser = validate_header, help = "key-value pair separated by a colon (:)")]
    header: Vec<Vec<RequestProperty>>,

    #[arg(long = "data-raw", help = "raw request body")]
    raw_data: Option<String>,

    #[arg(
        long,
        help = "this doesn't do anything, but is added for compatibility"
    )]
    compressed: bool,
}

fn validate_header(s: &str) -> Result<Vec<RequestProperty>, String> {
    let mut str_iter = s.splitn(2, ":");
    let key = str_iter
        .next()
        .ok_or(format!("{} is not a valid header", s))?
        .trim();
    let value = str_iter
        .next()
        .ok_or(format!("{} is not a valid header", s))?
        .trim();

    if key.to_lowercase() == "cookie" {
        return Ok(value
            .split(';')
            .map(|c| RequestProperty::Cookie {
                cookie: c.trim().to_string(),
            })
            .collect());
    }

    Ok(vec![RequestProperty::Header {
        key: key.to_string(),
        value: value.to_string(),
    }])
}

fn parse_queries(url: &Url) -> Vec<RequestProperty> {
    url.query_pairs()
        .map(|(key, value)| RequestProperty::QueryParameter {
            key: key.into_owned(),
            value: value.into_owned(),
        })
        .collect()
}

pub struct ParsedArgs {
    pub url: Url,
    pub method: Method,
    pub properties: Vec<RequestProperty>,
}

pub fn get_args() -> anyhow::Result<ParsedArgs> {
    let mut args = Args::parse();

    let queries = parse_queries(&args.url);
    let mut properties: Vec<RequestProperty> =
        args.header.into_iter().flatten().chain(queries).collect();

    if let Some(data) = args.raw_data {
        properties.push(RequestProperty::Body { body: data });
    }

    args.url.set_query(None);

    let method = match args.request {
        Methods::GET => Method::GET,
        Methods::POST => Method::POST,
        Methods::PUT => Method::PUT,
        Methods::DELETE => Method::DELETE,
        Methods::HEAD => Method::HEAD,
        Methods::OPTIONS => Method::OPTIONS,
        Methods::CONNECT => Method::CONNECT,
        Methods::PATCH => Method::PATCH,
        Methods::TRACE => Method::TRACE,
    };

    Ok(ParsedArgs {
        url: args.url,
        method,
        properties,
    })
}
