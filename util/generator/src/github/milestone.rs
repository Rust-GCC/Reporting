//! Module related to github milestones.
use crate::{
    error::Error,
    naming::{ORGANISATION, REPOSITORY},
};
use async_trait::async_trait;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter, Result as FmtResult};

use octocrab::{models::Milestone, Octocrab, Page};

#[async_trait]
trait MilestoneExt {
    async fn milestones(&self, owner: &str, repo: &str)
        -> Result<Page<Milestone>, octocrab::Error>;
}

#[async_trait]
impl MilestoneExt for Octocrab {
    async fn milestones(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Page<Milestone>, octocrab::Error> {
        self.get(format!("https://api.github.com/repos/{owner}/{repo}/milestones"), None::<&()>)
            .await
    }
}

pub struct MStone {
    title: String,
    open_issues: i64,
    closed_issues: i64,
    created_at: NaiveDate,
    closed_at: Option<NaiveDate>,
    due_on: Option<NaiveDate>,
}

impl From<Milestone> for MStone {
    fn from(ms: Milestone) -> MStone {
        MStone {
            title: ms.title,
            open_issues: ms.open_issues.unwrap(),
            closed_issues: ms.closed_issues.unwrap(),
            created_at: ms.created_at.date_naive(),
            // FIXME: This is bad: No unwrap. It's valid to not have one
            closed_at: ms.closed_at.map(|date| date.date_naive()),
            // FIXME: This is bad: No unwrap. It's valid to not have one
            due_on: ms.due_on.map(|date| date.date_naive()),
        }
    }
}

impl Display for MStone {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // TODO: Add checker that this is valid org-mode? unit tests?
        write!(f, "|{}|", self.title)
    }
}

/// Fetch all milestones
///
/// * `gh` - Octocrab instance that should be used to fetch data.
pub async fn fetch_all(gh: &Octocrab) -> Result<Vec<Milestone>, Error> {
    Ok(gh.milestones(ORGANISATION, REPOSITORY).await?.take_items())
}
