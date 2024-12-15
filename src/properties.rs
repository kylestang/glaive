use reqwest::RequestBuilder;

#[derive(Clone, Debug)]
pub enum RequestProperty {
    QueryParameter { key: String, value: String },
    Header { key: String, value: String },
    Body { body: String },
    Cookie { cookie: String },
}

pub fn synthesize_request(
    properties: &Vec<&RequestProperty>,
    mut builder: RequestBuilder,
) -> RequestBuilder {
    let mut cookies: Vec<&str> = Vec::new();

    for property in properties {
        match property {
            RequestProperty::QueryParameter { key, value } => {
                builder = builder.query(&[(key, value)])
            }
            RequestProperty::Header { key, value } => builder = builder.header(key, value),
            RequestProperty::Body { body } => builder = builder.body(body.clone()),
            RequestProperty::Cookie { cookie } => cookies.push(cookie),
        }
    }

    if !cookies.is_empty() {
        builder = builder.header("Cookie", cookies.join("; "));
    }

    builder
}
