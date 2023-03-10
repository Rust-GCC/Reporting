// TODO: Cleanup, improve
#[derive(Debug)]
pub enum Error {
    GitHubRequest(octocrab::Error),
    Template(tinytemplate::error::Error),
}

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
