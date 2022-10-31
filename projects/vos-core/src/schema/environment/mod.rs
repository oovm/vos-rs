use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    host: Url,
}
