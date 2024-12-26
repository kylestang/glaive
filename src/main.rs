use anyhow::anyhow;
use cli::ParsedArgs;
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
        Ok(properties) => println!("Required Properties: {properties:?}"),
        Err(a) => println!("Error: {}", a),
    };
}

async fn run_glaive(args: ParsedArgs) -> anyhow::Result<Vec<RequestProperty>> {
    let client = Client::builder().use_rustls_tls().build()?;

    let goal_request = synthesize_request(
        &args.properties,
        client.request(args.method.clone(), args.url.clone()),
    );
    let goal = send_request(
        goal_request
            .try_clone()
            .ok_or(anyhow!("look, this shouldn't have been possible. I don't know how it happened. Maybe take up knitting instead?"))?,
    )
    .await?;
    let sanity_check = send_request(goal_request).await?;
    if goal != sanity_check {
        return Err(anyhow!(
            "sanity check failed, two requests with all properties returned different results"
        ));
    }

    let mut required = args.properties.clone();
    let mut i = 0;

    while i < required.len() {
        let mut test = required.clone();
        test.remove(i);

        let test_req =
            synthesize_request(&test, client.request(args.method.clone(), args.url.clone()));

        let response = send_request(test_req).await?;
        if response == goal {
            required = test;
        } else {
            i += 1;
        }
    }

    Ok(required)
}

async fn send_request(request: RequestBuilder) -> anyhow::Result<ResponseCriteria> {
    let response = request.send().await?;
    Ok(ResponseCriteria {
        status_code: response.status(),
        body: response.text().await?,
    })
}
