use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Endpoint {
    pub request: String,
    pub response: String,
    pub get: Request,
    pub document: Document,
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Request {
    open: String,
}

impl Endpoint {
    pub fn http_get() {}
}
