
use std::fmt::{Display, Formatter, Result as FmtResult};

use octocrab::{models::pulls::PullRequest, params::State, Octocrab };

#[derive(Hash, Eq, PartialEq)]
pub struct Contrib {
    name: String,
    url: String,
}

impl From<PullRequest> for Contrib {
    fn from(pr: PullRequest) -> Contrib {
        let user = *(pr.user.unwrap());
        Contrib {
            name: user.login,
            url: user.url.to_string(),
        }
    }
}

impl Display for Contrib {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        write!(f, "[[{}][{}]]", self.name, self.url)
    }
}
