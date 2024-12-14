use cli::ParsedArgs;
use itertools::Itertools;
use properties::RequestProperty;
use reqwest::{Client, RequestBuilder, StatusCode, Url};

mod cli;
mod properties;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResponseCriteria {
    status_code: StatusCode,
    body: String,
}

#[tokio::main]
async fn main() {
    let args = cli::get_args().unwrap();

    match do_thing(args).await {
        Ok(criteria) => println!("Valid criteria: {criteria:?}"),
        Err(a) => println!("Error: {}", a),
    };
}

async fn do_thing(args: ParsedArgs) -> anyhow::Result<Option<Vec<RequestProperty>>> {
    let mut url = args.url.clone();
    let mut properties = parse_query(&url);
    properties.extend(args.headers);

    let client = Client::builder().use_rustls_tls().build()?;
    let goal = send_request(client.request(args.method.clone(), url.clone())).await?;

    url.set_query(None);

    for i in 0..=properties.len() {
        for combination in properties.iter().combinations(i) {
            let request = combination.iter().fold(
                client.request(args.method.clone(), url.clone()),
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
