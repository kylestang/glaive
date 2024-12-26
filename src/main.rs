use std::{collections::VecDeque, sync::Arc};

use cli::ParsedArgs;
use itertools::Itertools;
use properties::{synthesize_request, RequestProperty};
use reqwest::{Client, RequestBuilder, StatusCode};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

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

    let mut tasks = VecDeque::new();
    let semaphore = Arc::new(Semaphore::new(args.concurrency));

    for i in 0..=args.properties.len() {
        for combination in args.properties.iter().combinations(i) {
            let request = synthesize_request(
                &combination,
                client.request(args.method.clone(), args.url.clone()),
            );

            let permit = match semaphore.clone().acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => break,
            };
            tasks.push_back((
                combination,
                tokio::spawn(create_request(request, goal.clone(), permit)),
            ));
        }
        if semaphore.is_closed() {
            break;
        }
    }

    let mut answer = None;
    while let Some((combination, handle)) = tasks.pop_front() {
        if handle.await?? {
            answer = Some(combination.iter().map(|&c| c.clone()).collect());
            break;
        }
    }

    for (_, task) in tasks {
        task.abort();
    }

    Ok(answer)
}

async fn create_request(
    request: RequestBuilder,
    goal: ResponseCriteria,
    permit: OwnedSemaphorePermit,
) -> anyhow::Result<bool> {
    let response = send_request(request).await?;
    if response == goal {
        permit.semaphore().close();
        return Ok(true);
    }
    Ok(false)
}

async fn send_request(request: RequestBuilder) -> anyhow::Result<ResponseCriteria> {
    let response = request.send().await?;
    Ok(ResponseCriteria {
        status_code: response.status(),
        body: response.text().await?,
    })
}
