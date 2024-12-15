use cli::ParsedArgs;
use itertools::Itertools;
use properties::{synthesize_request, RequestProperty};
use reqwest::{Client, RequestBuilder, StatusCode};

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

    match run_glaive(args).await {
        Ok(criteria) => println!("Valid criteria: {criteria:?}"),
        Err(a) => println!("Error: {}", a),
    };
}

async fn run_glaive(args: ParsedArgs) -> anyhow::Result<Option<Vec<RequestProperty>>> {
    let client = Client::builder().use_rustls_tls().build()?;

    let goal_request = synthesize_request(
        &args.properties.iter().collect(),
        client.request(args.method.clone(), args.url.clone()),
    );
    let goal = send_request(goal_request).await?;

    for i in 0..=args.properties.len() {
        for combination in args.properties.iter().combinations(i) {
            let request = synthesize_request(
                &combination,
                client.request(args.method.clone(), args.url.clone()),
            );

            let response = send_request(request).await?;
            if response == goal {
                return Ok(Some(combination.iter().map(|&x| x.clone()).collect()));
            }
        }
    }

    Ok(None)
}

async fn send_request(request: RequestBuilder) -> anyhow::Result<ResponseCriteria> {
    let response = request.send().await?;
    Ok(ResponseCriteria {
        status_code: response.status(),
        body: response.text().await?,
    })
}
