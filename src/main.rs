use clap::Parser;
use itertools::Itertools;
use reqwest::{Client, Method, RequestBuilder, StatusCode, Url};

#[derive(Clone, Debug)]
enum RequestProperty {
    QueryParameter { key: String, value: String },
    Header { key: String, value: String },
    Body { body: String },
}

impl RequestProperty {
    fn add_to_request(&self, builder: RequestBuilder) -> RequestBuilder {
        match self {
            RequestProperty::QueryParameter { key, value } => builder.query(&[(key, value)]),
            RequestProperty::Header { key, value } => builder.header(key, value),
            RequestProperty::Body { body } => builder.body(body.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResponseCriteria {
    status_code: StatusCode,
    body: String,
}

#[derive(Parser)]
struct Args {
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match do_thing(args).await {
        Ok(criteria) => println!("Valid criteria: {criteria:?}"),
        Err(a) => println!("Error: {}", a),
    };
}

async fn do_thing(args: Args) -> anyhow::Result<Option<Vec<RequestProperty>>> {
    let mut url = Url::parse(&args.url)?;
    let properties = parse_query(&url);

    let client = Client::builder().use_rustls_tls().build()?;
    let goal = send_request(client.request(Method::GET, url.clone())).await?;

    url.set_query(None);

    for i in 0..=properties.len() {
        for combination in properties.iter().combinations(i) {
            let request = combination.iter().fold(
                client.request(Method::GET, url.clone()),
                |builder, property| property.add_to_request(builder),
            );

            let response = send_request(request).await?;
            if response == goal {
                return Ok(Some(combination.iter().map(|&x| x.clone()).collect()));
            }
        }
    }

    Ok(None)
}

fn parse_query(url: &Url) -> Vec<RequestProperty> {
    url.query_pairs()
        .map(|(key, value)| RequestProperty::QueryParameter {
            key: key.into_owned(),
            value: value.into_owned(),
        })
        .collect()
}

async fn send_request(request: RequestBuilder) -> anyhow::Result<ResponseCriteria> {
    let response = request.send().await?;
    Ok(ResponseCriteria {
        status_code: response.status(),
        body: response.text().await?,
    })
}
