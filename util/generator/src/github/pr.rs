use crate::{
    error::Error,
    naming::{ORGANISATION, REPOSITORY},
};
use chrono::NaiveDate;
use octocrab::{models::pulls::PullRequest, params::State, Octocrab};

use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Pr {
    number: u64,
    url: String,
    title: String,
}

impl From<PullRequest> for Pr {
    fn from(pr: PullRequest) -> Pr {
        Pr {
            number: pr.number,
            url: pr.url,
            // FIXME: This contains HTML character references, like &#39
            // Figure out how to remove them. This is blocking for this to be used
            title: pr.title.unwrap(),
        }
    }
}

impl Display for Pr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // TODO: Add checker that this is valid org-mode? unit tests?
        write!(f, "- {} [[{}][PR{}]]", self.title, self.url, self.number)
    }
}

/// Fetch all merged pull requests merged a week or a month before the given date
pub async fn fetch_merged(
    gh: &Octocrab,
    from: &NaiveDate,
    to: &NaiveDate,
) -> Result<Vec<PullRequest>, Error> {
    let mut pages = gh
        .pulls(ORGANISATION, REPOSITORY)
        .list()
        .state(State::Closed)
        .per_page(100)
        .send()
        .await?;

    Ok(pages
        .take_items()
        .into_iter()
        .fold(Vec::new(), |mut acc, pr| {
            if let Some(merge_date) = pr.merged_at {
                let merge_date = merge_date.date_naive();
                // Is the inclusive range okay?
                if &merge_date >= from && &merge_date <= to {
                    dbg!(&pr.url);
                    acc.push(pr)
                }
            }

            acc
        }))
}
