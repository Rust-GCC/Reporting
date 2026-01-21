//! This utilitary generates a barebone report based on a given date and timespan (weekly, monthly).
//! Its goal is to help with things such as fetching merged PRs from github, generating milestone tables,
//! fetching bug/issue status, and so on.

use clap::{Parser, ValueEnum};
use error::Error;
use git2::{Oid, Repository};
use indicatif::MultiProgress;
use octocrab::OctocrabBuilder;
use progress::{FetchMethod, ProgressBarExt, Steps};
use serde::Serialize;
use std::path::PathBuf;
use testcase::TestCases;
use tinytemplate::TinyTemplate;
use std::collections::HashSet;

use chrono::{Days, Months, NaiveDate};

use github::{
    milestone::{self, MStone},
    pr::{self, find_oldest_pr_merge_commit, Pr},
    contrib::{self, Contrib},
    issues::{self, get_TODO, get_closed, get_in_progress}
};

use task::Task;

mod naming {
    pub const ORGANISATION: &str = "rust-gcc";
    pub const REPOSITORY: &str = "gccrs";
}

mod error;
mod github;
mod progress;
mod repository;
mod testcase;
mod task;

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
    let gh = OctocrabBuilder::new().build()?;
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

    let old_commit_sha = find_oldest_pr_merge_commit(&gh, &merged_prs).await;

    let ctb = pr::fetch_merged(&gh, &from_date, &args.date)
        .await?
        // FIXME: Would it be better to have a trait extension for octo::PullRequest here?
        // and `impl ReportFormatter` on it?
        // yes
        .into_iter()
        .map(Contrib::from)
        .collect::<HashSet<Contrib>>();


    let test_cases = if let (Some(old_commit_sha), false) = (old_commit_sha, args.skip_tests) {

        let repository = if let Some(ref path) = args.local_repo {
            bar.set_step(Steps::FetchRepository(FetchMethod::Local));
            Repository::open(path).map_err(Error::Repository)?
        } else {
            bar.set_step(Steps::FetchRepository(FetchMethod::Remote));
            repository::clone(&gh, &multi).await.map_err(Error::Clone)?
        };

        let previous_commit = repository
            .find_commit(
                Oid::from_str(&old_commit_sha).expect("We found this SHA in the repository"),
            )
            // TODO: Cleanup
            .map_err(|_| Error::RepoNotUpToDate("Be sur that your repo is up do date"))?
            .parent_id(0)
            .unwrap();

        let results = TestCases::collect(
            &repository,
            &previous_commit.to_string(),
            "master", /* TODO: Change this value ? Do we need to generate older reports ? */
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
    } else {
        println!("Tests are skipped ? {}", args.skip_tests);
        String::new()
    };


    bar.set_step(Steps::RetrieveMilestones);
    let milestones = milestone::fetch_all(&gh)
        .await?
        .into_iter()
        .map(MStone::from)
        .collect::<Vec<MStone>>();

    let nb_todo = get_TODO(&gh, false).await?;
    let in_prog = get_in_progress(&gh, false).await?;
    let closed = get_closed(&gh, false).await?;

    let task_status =  Task{nb_todo, in_prog, closed}.to_string();

    let nb_todo = get_TODO(&gh, true).await?;
    let in_prog = get_in_progress(&gh, true).await?;
    let closed = get_closed(&gh, true).await?;

    let bugs =  Task{nb_todo, in_prog, closed}.to_string();
    
    
    let ctx = Context {
        kind: args.kind,
        from: from_date,
        to: args.date,
        author: args.author,
        date: args.date,
        contributors: ctb.iter().fold(String::new(), |acc, ct| format!("{acc}\n{ct}")),
        task_status,
        test_cases,
        bugs,
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
