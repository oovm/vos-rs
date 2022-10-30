use super::*;
use vos_error::VosResult;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ProjectAuthor {
    pub name: String,
    /// email is the user's primary id
    pub email: EmailAddress,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Object>,
}

impl Eq for ProjectAuthor {}

impl PartialEq<Self> for ProjectAuthor {
    fn eq(&self, other: &Self) -> bool {
        if self.email.is_none() || other.email.is_none() { self.name.eq(&other.name) } else { self.email.eq(&other.email) }
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
    pub fn email(email: &str) -> VosResult<EmailAddress> {}

    pub fn short_name(&self) -> bool {
        todo!()
    }
}
