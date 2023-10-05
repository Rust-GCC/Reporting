//! This utilitary generates a barebone report based on a given date and timespan (weekly, monthly).
//! Its goal is to help with things such as fetching merged PRs from github, generating milestone tables,
//! fetching bug/issue status, and so on.

use clap::{Parser, ValueEnum};
use error::Error;
use git2::Repository;
use indicatif::MultiProgress;
use progress::{FetchMethod, ProgressBarExt, Steps};
use serde::Serialize;
use std::path::PathBuf;
use testcase::TestCases;
use tinytemplate::TinyTemplate;

use chrono::{Days, Months, NaiveDate};

use github::{
    milestone::{self, MStone},
    pr::{self, Pr},
};

mod naming {
    pub static ORGANISATION: &str = "rust-gcc";
    pub static REPOSITORY: &str = "gccrs";
}

mod error;
mod github;
mod progress;
mod repository;
mod testcase;

// FIXME: There should be two templates: one for weekly reports, one for monthly reports, as monthly reports include tests etc
static TEMPLATE: &str = include_str!("templates/report.org.template");

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
    #[arg(short, long, help = "Skip tests and do not report deltas")]
    pub skip_tests: bool,
    #[arg(
        short,
        long,
        help = "Path to a local repository to avoid cloning the project"
    )]
    pub local_repo: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "Additional arguments to pass to the configure script"
    )]
    pub configure: Option<String>,
    #[arg(short, long)]
    pub kind: Kind,
    #[arg(short, long)]
    pub date: NaiveDate,
    #[arg(short, long)]
    pub author: String,
}

fn get_from_date(kind: &Kind, date: &NaiveDate) -> NaiveDate {
    match kind {
        Kind::Weekly => date.checked_sub_days(Days::new(7)).unwrap(),
        Kind::Monthly => date.checked_sub_months(Months::new(1)).unwrap(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let gh = octocrab::instance();
    let from_date = get_from_date(&args.kind, &args.date);

    let multi = MultiProgress::new();
    let bar = progress::default_progress_bar();
    multi.add(bar.clone());

    bar.set_step(progress::Steps::RetrievePrs);
    let merged_prs = pr::fetch_merged(&gh, &from_date, &args.date)
        .await?
        // FIXME: Would it be better to have a trait extension for octo::PullRequest here?
        // and `impl ReportFormatter` on it?
        // yes
        .into_iter()
        .map(Pr::from)
        .collect::<Vec<Pr>>();

    let test_cases = if args.skip_tests {
        String::new()
    } else {
        let repository = if let Some(ref path) = args.local_repo {
            bar.set_step(Steps::FetchRepository(FetchMethod::Local));
            Repository::open(path).map_err(Error::Repository)?
        } else {
            bar.set_step(Steps::FetchRepository(FetchMethod::Remote));
            repository::clone(&gh, &multi).await.map_err(Error::Clone)?
        };

        // FIXME: Change "master" references to actual references
        // Should we rely on PRs or git history ?
        let results = TestCases::collect(
            &repository,
            "master",
            "master",
            &args.configure.unwrap_or_default(),
        )
        .await
        .map_err(Error::Test)?;

        if args.local_repo.is_none() {
            bar.set_step(Steps::Cleaning);
            std::fs::remove_dir_all(
                repository
                    .path()
                    .parent()
                    .expect(".git parent should always exist"),
            )
            .map_err(Error::Workspace)?;
        }

        results.to_string()
    };

    bar.set_step(Steps::RetrieveMilestones);
    let milestones = milestone::fetch_all(&gh)
        .await?
        .into_iter()
        .map(MStone::from)
        .collect::<Vec<MStone>>();

    let ctx = Context {
        kind: args.kind,
        from: from_date,
        to: args.date,
        author: args.author,
        date: args.date,
        contributors: String::new(),
        task_status: String::new(),
        test_cases,
        bugs: String::new(),
        milestones: milestones.iter().fold(String::new(), |acc, milestone| {
            format!("{acc}\n{milestone}")
        }),
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
