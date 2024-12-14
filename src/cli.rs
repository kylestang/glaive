use clap::{Parser, ValueEnum};
use reqwest::{Method, Url};
use std::fmt::Display;

use crate::RequestProperty;

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

    #[arg(short = 'X', long, default_value_t = Methods::GET)]
    request: Methods,

    #[arg(short = 'H', long, value_parser = validate_header, help = "key-value pair separated by a colon (:)")]
    header: Vec<RequestProperty>,
}

fn validate_header(s: &str) -> Result<RequestProperty, String> {
    let mut str_iter = s.splitn(2, ":");

    Ok(RequestProperty::Header {
        key: str_iter
            .next()
            .ok_or(format!("{} is not a valid header", s))?
            .trim()
            .to_string(),
        value: str_iter
            .next()
            .ok_or(format!("{} is not a valid header", s))?
            .trim()
            .to_string(),
    })
}

pub struct ParsedArgs {
    pub url: Url,
    pub method: Method,
    pub headers: Vec<RequestProperty>,
}

pub fn get_args() -> anyhow::Result<ParsedArgs> {
    let args = Args::parse();

    Ok(ParsedArgs {
        url: args.url,
        method: match args.request {
            Methods::GET => Method::GET,
            Methods::POST => Method::POST,
            Methods::PUT => Method::PUT,
            Methods::DELETE => Method::DELETE,
            Methods::HEAD => Method::HEAD,
            Methods::OPTIONS => Method::OPTIONS,
            Methods::CONNECT => Method::CONNECT,
            Methods::PATCH => Method::PATCH,
            Methods::TRACE => Method::TRACE,
        },
        headers: args.header,
    })
}
