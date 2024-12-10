use crate::repository::CloneError;
use crate::testcase::ReportError;
use std::{fmt, io};

// TODO: Cleanup, improve
#[derive(Debug)]
pub enum Error {
    GitHubRequest(octocrab::Error),
    Template(tinytemplate::error::Error),
    Clone(CloneError),
    Test(ReportError),
    Workspace(io::Error),
    Repository(git2::Error),
    RepoNotUpToDate(& 'static str),
    SerdeNotFound(& 'static str)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            GitHubRequest(why) => write!(f, "Error whilst contacting github: {why}"),
            Template(why) => write!(f, "Templating error: {why}"),
            Clone(why) => write!(f, "Clone error: {why}"),
            Test(why) => write!(f, "Testsuite error: {why}"),
            Workspace(why) => write!(f, "Workspace management error: {why}"),
            Repository(why) => write!(f, "Repository management error: {why}"),
            RepoNotUpToDate(why) => write!(f, "{why}"),
            SerdeNotFound(why) => write!(f, "Field {why} not found")
        }
    }
}

impl std::error::Error for Error {}

impl From<octocrab::Error> for Error {
    fn from(e: octocrab::Error) -> Error {
        Error::GitHubRequest(e)
    }
}

impl From<tinytemplate::error::Error> for Error {
    fn from(e: tinytemplate::error::Error) -> Error {
        Error::Template(e)
    }
}
