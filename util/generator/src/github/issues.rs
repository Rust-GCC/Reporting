//! Module related to github pull requests.
use crate::{
    error::Error,
    naming::{ORGANISATION, REPOSITORY},
};
use octocrab::{models::{issues::Issue, IssueState}, Octocrab, Page};
use serde::Serialize;

use std::fmt::{Display, Formatter, Result as FmtResult};

use serde_json::Value;

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

pub async fn get_in_progress(
     gh: &Octocrab,
     bug: bool
 ) -> Result<u64, Error> {

     let mut all_uri = String::from("https://api.github.com/search/issues?q=repo:Rust-GCC/gccrs+type:issue+state:open");
     let mut no_assignee_uri =  String::from("https://api.github.com/search/issues?q=repo:Rust-GCC/gccrs+type:issue+state:open+no:assignee");

     if bug
     {
         all_uri.push_str("+label:bug");    
         no_assignee_uri.push_str("+label:bug");
     }

    let all = gh._get(all_uri).await.unwrap();
    let no_assignee = gh._get(no_assignee_uri).await.unwrap();

    let serres: Value = serde_json::from_str(gh.body_to_string(all).await.unwrap().as_str())?;
    let serres_no: Value  = serde_json::from_str(gh.body_to_string(no_assignee).await.unwrap().as_str())?;

    let nb_all = if let Some(x) = serres["total_count"].as_u64() 
    {
        x
    }
    else
    {
        0
    };
    let nb_no = if let Some(x) = serres_no["total_count"].as_u64() 
    {
        x
    }
    else
    {
        0
    };

    Ok(nb_all - nb_no)
    }

    pub async fn get_TODO(
        gh: &Octocrab,
        bug :bool
    ) -> Result<u64, Error> {
    
    let mut no_assignee_uri = String::from("https://api.github.com/search/issues?q=repo:Rust-GCC/gccrs+type:issue+state:open+no:assignee");

    if bug
    {
        no_assignee_uri.push_str("+label:bug")
    }

    let no_assignee = gh._get(no_assignee_uri).await.unwrap();

    let serres_no: Value  = serde_json::from_str(gh.body_to_string(no_assignee).await.unwrap().as_str())?;

    let nb_no = if let Some(x) = serres_no["total_count"].as_u64() 
    {
        x
    }
    else
    {
        0
    };

    Ok(nb_no)
    }

    pub async fn get_closed(
        gh: &Octocrab,
        bug: bool
    ) -> Result<u64, Error> {
    
    let mut no_assignee_uri = String::from("https://api.github.com/search/issues?q=repo:Rust-GCC/gccrs+type:issue+state:closed");

    if bug
    {
        no_assignee_uri.push_str("+label:bug")
    }

    let no_assignee = gh._get(no_assignee_uri).await.unwrap();

    let serres_no: Value  = serde_json::from_str(gh.body_to_string(no_assignee).await.unwrap().as_str())?;

    let nb_no = if let Some(x) = serres_no["total_count"].as_u64() 
    {
        x
    }
    else
    {
        0
    };

    Ok(nb_no)
    }
