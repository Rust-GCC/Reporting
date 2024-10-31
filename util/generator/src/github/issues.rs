//! Module related to github pull requests.
use crate::{
    error::Error,
    naming::{ORGANISATION, REPOSITORY},
};
use octocrab::{models::{issues::Issue, IssueState}, Octocrab, Page};
use serde::Serialize;

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug)]
pub struct Issues{
    number: u64,
    title: String,
    pub state: IssueState,
}

impl From<Issue> for Issues{
    fn from(is: Issue) -> Issues{
        Issues {
            number: is.number,
            title: is.title,
            state: is.state,
        }
    }
}

impl Display for Issues{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.title)
    }
}

/// Fetch all issues
///
/// # Arguments
///
/// * `gh` - Octocrab instance that should be used to fetch data.
pub async fn fetch_issues(
    gh: &Octocrab,
) -> Result<Page<Issue>, Error> {
    let mut pages = gh
        .issues(ORGANISATION, REPOSITORY)
        .list()
        .state(octocrab::params::State::Closed)
        .per_page(100)
        .send()
        .await?;
    Ok(pages)
}

pub async fn get_nb_closed_issues(
     gh: &Octocrab,
 ) -> Result<u32, Error> {
    
    let mut pages = gh
        .issues(ORGANISATION, REPOSITORY)
        .list()
        .state(octocrab::params::State::Closed)
        .per_page(100)
        .send()
        .await?;
    let nb_p : u32 = pages.number_of_pages().unwrap();

    let mut last_page = gh
        .issues(ORGANISATION, REPOSITORY)
        .list()
        .state(octocrab::params::State::Closed)
        .per_page(100)
        .page(nb_p)
        .send()
        .await?;

    let response = gh._get("https://api.github.com/search/issues?q=repo:Rust-GCC/gccrs+type:issue+state:closed").await.unwrap();

    dbg!(&response);


    let nb_issues_last_p : u32 = last_page.into_iter().count().try_into().unwrap();
    Ok((nb_p - 1) * 50+ nb_issues_last_p)
    }
