use super::*;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ProjectAuthor {
    /// If the user is already registered, a uuid will be assigned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Object>,
}

impl Default for ProjectAuthor {
    fn default() -> Self {
        Self { name: "".to_string(), user: Uuid::from_u128(0), email: None, homepage: None, extra: Default::default() }
    }
}

impl Eq for ProjectAuthor {}

impl PartialEq<Self> for ProjectAuthor {
    fn eq(&self, other: &Self) -> bool {
        if self.user.is_nil() || other.user.is_nil() { self.name.eq(&other.name) } else { self.user.eq(&other.user) }
    }
}

impl PartialOrd<Self> for ProjectAuthor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for ProjectAuthor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl ProjectAuthor {
    pub fn only_name(&self) -> bool {
        self.user.is_nil() && self.email.is_none() && self.extra.is_empty() && self.homepage.is_none()
    }
}
