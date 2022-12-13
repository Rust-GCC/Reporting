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
    // TODO: Figure out how to unnest this; doing it just by creating another structure does not work.
    // Probably needs two template: One for titleblock and one for the template, which include a title block
    // so have another static TITLE_BLOCK_TEMPLATE: &str = "#+title: {kind} report for..." etc
    kind: Kind,
    from: NaiveDate,
    to: NaiveDate,
    author: String,
    date: NaiveDate,
    merged_prs: String,
    contributors: String,
    task_status: String,
    test_cases: String,
    bugs: String,
    milestones: String,
}

#[derive(ValueEnum, Clone, Copy, Serialize)]
enum Kind {
    // TODO: Unit test to make sure this keeps being formatted as "Weekly" and "Monthly" in rendered reports
    Weekly,
    Monthly,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pub kind: Kind,
    #[arg(short, long)]
    pub date: NaiveDate,
    #[arg(short, long)]
    pub author: String,
}

// TODO: Cleanup, improve
#[derive(Debug)]
enum Error {
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

fn get_from_date(kind: &Kind, date: &NaiveDate) -> NaiveDate {
    match kind {
        Kind::Weekly => date.checked_sub_days(Days::new(7)).unwrap(),
        Kind::Monthly => date.checked_sub_months(Months::new(1)).unwrap(),
    }
}

/// Fetch all merged pull requests merged a week or a month before the given date
async fn fetch_merged_prs(
    gh: &Octocrab,
    from: &NaiveDate,
    to: &NaiveDate,
) -> Result<Vec<PullRequest>, Error> {
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
                if &merge_date >= from && &merge_date <= to {
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let gh = octocrab::instance();
    let from_date = get_from_date(&args.kind, &args.date);

    let merged_prs = fetch_merged_prs(&gh, &from_date, &args.date)
        .await?
        // FIXME: Would it be better to have a trait extension for octo::PullRequest here?
        // and `impl ReportFormatter` on it?
        .into_iter()
        .map(Pr::from)
        .collect::<Vec<Pr>>();

    let ctx = Context {
        kind: args.kind,
        from: from_date,
        to: args.date,
        author: args.author,
        date: args.date,
        contributors: String::new(),
        task_status: String::new(),
        test_cases: String::new(),
        bugs: String::new(),
        milestones: String::new(),
        merged_prs: merged_prs
            .iter()
            .fold(String::new(), |acc, pr| format!("{acc}\n{pr}")),
    };

    let mut renderer = TinyTemplate::new();
    renderer.add_template("report", TEMPLATE)?;

    let rendered = renderer.render("report", &ctx)?;

    println!("{rendered}");

    Ok(())
}
