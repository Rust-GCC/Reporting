//! Module related to github pull requests.
use crate::{
    error::Error,
    naming::{ORGANISATION, REPOSITORY},
};
use chrono::{offset::Utc, DateTime, NaiveDate};
use octocrab::{models::pulls::PullRequest, params::State, Octocrab};

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug)]
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
///
/// # Arguments
///
/// * `gh` - Octocrab instance that should be used to fetch data.
/// * `from` - Start date for the time period containing the merged pull requests.
/// * `to` - End date for the time period containing the merged pull requests.
pub async fn fetch_merged(
    gh: &Octocrab,
    from: &NaiveDate,
    to: &NaiveDate,
) -> Result<Vec<PullRequest>, Error> {
    let mut pages = gh
        .pulls(ORGANISATION, REPOSITORY)
        .list()
        .state(State::All)
        .per_page(100)
        .send()
        .await?;


    Ok(pages
        .take_items()
        .into_iter()
        .filter(|pr| {
            pr.merged_at
                .map(|e| {
                    let merge_date = &e.date_naive();
                    // Is the inclusive range okay?
                    merge_date > from && merge_date < to
                })
                .unwrap_or(false)
        })
        .collect())
}

pub async fn find_oldest_pr_merge_commit(gh: &Octocrab, prs: &[Pr]) -> Option<String> {
    let mut oldest: Option<(DateTime<Utc>, Option<String>)> = None;
    for pr in prs {
        let complete_pr = match gh.pulls(ORGANISATION, REPOSITORY).get(pr.number).await {
            Ok(x) if x.merged_at.is_some() => x,
            _ => continue,
        };
        oldest = match oldest {
            Some(ref old_date) if old_date.0 < complete_pr.merged_at.unwrap() => oldest,
            // None or older
            _ => Some((complete_pr.merged_at.unwrap(), complete_pr.merge_commit_sha)),
        }
    }
    oldest.map(|(_, pr)| pr).flatten()
}
