use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    pub host: Url,
    pub document: Document,
}

impl Eq for Environment {}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        self.host.eq(&other.host)
    }
}

impl PartialOrd for Environment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.host.partial_cmp(&other.host)
    }
}

impl Ord for Environment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.host.cmp(&other.host)
    }
}

impl Environment {
    pub fn new(host: Url) -> Self {
        Self { host, document: Default::default() }
    }
}

impl Project {
    pub fn environment(&mut self, environment: Environment) {
        self.environments.insert(environment);
    }
}
