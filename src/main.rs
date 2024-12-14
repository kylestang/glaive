use clap::Parser;
use itertools::Itertools;
use reqwest::{Client, Method, Request, StatusCode, Url};

#[derive(Parser)]
struct Args {
    url: String,
}

#[derive(Clone, Eq, PartialEq)]
struct ResponseData {
    status: StatusCode,
    body: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match do_thing(args).await {
        Ok(()) => {}
        Err(a) => println!("Error: {}", a),
    };
}

async fn do_thing(args: Args) -> anyhow::Result<()> {
    let url = Url::parse(&args.url)?;
    let client = Client::builder().build()?;
    let original_res = client.request(Method::GET, url.clone()).send().await?;

    let response_data = ResponseData {
        status: original_res.status(),
        body: Vec::from(original_res.bytes().await?),
    };

    let mut clean_url = url.clone();
    clean_url.set_query(None);
    let clean_req = Request::new(Method::GET, clean_url);

    let suc_req = iterate_params(
        Request::new(Method::GET, url),
        clean_req,
        &response_data,
        &client,
    )
    .await?;

    if let Some(a) = suc_req {
        println!("{:?}", a);
    }

    Ok(())
}

async fn check_req(
    client: &Client,
    request: Request,
    expected: &ResponseData,
) -> anyhow::Result<bool> {
    let res = client.execute(request).await?;

    let response_data = ResponseData {
        status: res.status(),
        body: Vec::from(res.bytes().await?),
    };

    Ok(response_data == *expected)
}

async fn iterate_params(
    orig_req: Request,
    clean_req: Request,
    original_data: &ResponseData,
    client: &Client,
) -> anyhow::Result<Option<Request>> {
    let attributes: Vec<(String, String)> = orig_req.url().query_pairs().into_owned().collect();

    let mut combo = None;

    for num_attributes in 1..=attributes.len() {
        for attribute_set in attributes.iter().combinations(num_attributes) {
            let mut new_req = clone_request(&clean_req)?;

            new_req
                .url_mut()
                .query_pairs_mut()
                .extend_pairs(attribute_set);

            if check_req(client, clone_request(&new_req)?, original_data).await? {
                combo = Some(new_req);
                break;
            }
        }
        if combo.is_some() {
            break;
        }
    }

    Ok(combo)
}

#[inline]
fn clone_request(request: &Request) -> anyhow::Result<Request> {
    request
        .try_clone()
        .ok_or(anyhow::anyhow!("couldn't clone request"))
}
