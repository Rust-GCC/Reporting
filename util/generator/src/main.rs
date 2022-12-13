//! This utilitary generates a barebone report based on a given date and timespan (weekly, monthly).
//! Its goal is to help with things such as fetching merged PRs from github, generating milestone tables,
//! fetching bug/issue status, and so on.

use chrono::{Days, Months, NaiveDate};
use clap::{Parser, ValueEnum};
use octocrab::{models::pulls::PullRequest, params::State, Octocrab};
use serde::Serialize;
use tinytemplate::TinyTemplate;

use std::fmt::{Display, Formatter, Result as FmtResult};

static TEMPLATE: &str = include_str!("templates/report.org");

#[derive(Serialize)]
struct Context {
    title_block: String,
    merged_prs: String,
    contributors: String,
    task_status: String,
    test_cases: String,
    bugs: String,
    milestones: String,
}

#[derive(ValueEnum, Clone, Copy)]
enum Kind {
    Weekly,
    Monthly,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pub kind: Kind,
    #[arg(short, long)]
    pub date: NaiveDate,
}

// TODO: Cleanup, improve
#[derive(Debug)]
enum Error {
    GitHubRequest,
    Template,
}

impl From<octocrab::Error> for Error {
    fn from(_: octocrab::Error) -> Error {
        Error::GitHubRequest
    }
}

impl From<tinytemplate::error::Error> for Error {
    fn from(_: tinytemplate::error::Error) -> Error {
        Error::Template
    }
}

/// Fetch all merged pull requests merged a week or a month before the given date
async fn fetch_merged_prs(
    gh: &Octocrab,
    kind: &Kind,
    date: &NaiveDate,
) -> Result<Vec<PullRequest>, Error> {
    // FIXME: No unwrap
    let from = match kind {
        Kind::Weekly => date.checked_sub_days(Days::new(7)).unwrap(),
        Kind::Monthly => date.checked_sub_months(Months::new(1)).unwrap(),
    };

    let mut pages = gh
        .pulls("rust-gcc", "gccrs")
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
                if merge_date >= from && merge_date <= *date {
                    dbg!(&pr.url);
                    acc.push(pr)
                }
            }

            acc
        }))
}

struct Pr {
    number: u64,
    url: String,
    title: String,
}

impl From<PullRequest> for Pr {
    fn from(pr: PullRequest) -> Pr {
        Pr {
            number: pr.number,
            url: pr.url,
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let gh = octocrab::instance();
    let mut renderer = TinyTemplate::new();

    renderer.add_template("report", TEMPLATE)?;

    let merged_prs = fetch_merged_prs(&gh, &args.kind, &args.date)
        .await?
        // FIXME: Would it be better to have a trait extension for octo::PullRequest here?
        // and `impl ReportFormatter` on it?
        .into_iter()
        .map(Pr::from)
        .collect::<Vec<Pr>>();

    let ctx = Context {
        title_block: String::new(),
        contributors: String::new(),
        task_status: String::new(),
        test_cases: String::new(),
        bugs: String::new(),
        milestones: String::new(),
        merged_prs: merged_prs
            .iter()
            .fold(String::new(), |acc, pr| format!("{acc}\n{pr}")),
    };

    let rendered = renderer.render("report", &ctx)?;

    println!("{rendered}");

    Ok(())
}
