use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub host: Url,
    pub document: Document,
}

impl Eq for Environment {}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd for Environment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Environment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Environment {
    pub fn new(host: Url) -> Self {
        Self { name: "default".to_string(), host, document: Default::default() }
    }
}

impl Project {
    /// Add a new environment to the project
    pub fn environment(&mut self, environment: Environment) {
        self.environments.insert(environment);
    }
}
