use reqwest::RequestBuilder;

#[derive(Clone, Debug)]
pub enum RequestProperty {
    QueryParameter { key: String, value: String },
    Header { key: String, value: String },
    Body { body: String },
}

impl RequestProperty {
    pub fn add_to_request(&self, builder: RequestBuilder) -> RequestBuilder {
        match self {
            RequestProperty::QueryParameter { key, value } => builder.query(&[(key, value)]),
            RequestProperty::Header { key, value } => builder.header(key, value),
            RequestProperty::Body { body } => builder.body(body.clone()),
        }
    }
}
