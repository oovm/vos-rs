use std::cmp::Ordering;

use super::*;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ProjectAuthor {
    user_id: u64,
    name: String,
    email: Option<EmailAdree>,
    homepage: Option<url>,
}

impl Eq for ProjectAuthor {}

impl PartialEq<Self> for ProjectAuthor {
    fn eq(&self, other: &Self) -> bool {
        if self.user_id == 0 || other.user_id == 0 { self.name.eq(&other.name) } else { self.user_id.eq(&other.user_id) }
    }
}

impl PartialOrd<Self> for ProjectAuthor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.user_id == 0 || other.user_id == 0 {
            self.name.partial_cmp(&other.name)
        }
        else {
            self.user_id.partial_cmp(&other.user_id)
        }
    }
}

impl Ord for ProjectAuthor {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.user_id == 0 || other.user_id == 0 { self.name.cmp(&other.name) } else { self.user_id.cmp(&other.user_id) }
    }
}
