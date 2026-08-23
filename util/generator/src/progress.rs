use std::borrow::Cow;

use indicatif::{ProgressBar, ProgressStyle};

#[derive(Copy, Clone)]
pub enum FetchMethod {
    Local,
    Remote,
}

#[derive(Copy, Clone)]
pub enum Steps {
    RetrievePrs,
    FetchRepository(FetchMethod),
    PreviousTests,
    CurrentTests,
    Cleaning,
    RetrieveMilestones,
    CollectResults,
    _LAST,
}

impl From<Steps> for Cow<'static, str> {
    fn from(value: Steps) -> Self {
        match value {
            Steps::RetrievePrs => Cow::Borrowed("Retrieving merged PRs"),
            Steps::FetchRepository(FetchMethod::Local) => Cow::Borrowed("Locating repository"),
            Steps::FetchRepository(FetchMethod::Remote) => Cow::Borrowed("Cloning repository"),
            Steps::PreviousTests => Cow::Borrowed("Running previous tests"),
            Steps::CurrentTests => Cow::Borrowed("Running current tests"),
            Steps::Cleaning => Cow::Borrowed("Cleaning up"),
            Steps::RetrieveMilestones => Cow::Borrowed("Retrieving milestones"),
            Steps::CollectResults => Cow::Borrowed("Collect results"),
            Steps::_LAST => unreachable!(),
        }
    }
}

impl From<Steps> for u64 {
    fn from(value: Steps) -> Self {
        match value {
            Steps::RetrievePrs => 1,
            Steps::FetchRepository(_) => 2,
            Steps::PreviousTests => 3,
            Steps::CurrentTests => 4,
            Steps::Cleaning => 5,
            Steps::RetrieveMilestones => 6,
            Steps::CollectResults => 7,
            Steps::_LAST => 8,
        }
    }
}

pub fn default_progress_bar() -> ProgressBar {
    let bar = ProgressBar::new(Steps::RetrieveMilestones.into());
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );
    bar
}

pub trait ProgressBarExt {
    fn set_step(&self, step: Steps);
}

impl ProgressBarExt for ProgressBar {
    fn set_step(&self, step: Steps) {
        self.set_message(step);
        self.set_position(step.into());
    }
}
