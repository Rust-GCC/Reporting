//! Functional testsuite report collection module.
use crate::repository;
use git2::Repository;
use std::{
    fmt, fs, io,
    path::Path,
    process::{Command, Stdio},
    str::FromStr,
};

/// An enum representing the different error case during reporting phase.
#[derive(Debug)]
pub enum ReportError {
    /// A git2 error happened during commit checkout.
    CheckoutError(git2::Error),
    /// An error happened during workspace operations.
    LocalError(io::Error),
    /// An error happened during configuration stage.
    Configure(io::Error),
    /// An error happened during build stage.
    Make(io::Error),
    /// An error happened during check stage.
    Check(io::Error),
    /// Cannot convert program output to utf-8.
    CollectionError(std::str::Utf8Error),
}

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ReportError::*;
        match self {
            CheckoutError(why) => write!(f, "Error whilst checking out: {why}"),
            LocalError(why) => write!(f, "Local error: {why}"),
            Configure(why) => write!(f, "Error whilst running configure script: {why}"),
            Make(why) => write!(f, "Error whilst building compiler: {why}"),
            Check(why) => write!(f, "Error whilst running testsuite: {why}"),
            CollectionError(why) => write!(f, "Error whilst collecting testsuite output: {why}"),
        }
    }
}

impl std::error::Error for ReportError {}

/// A number of passed of failed tests with a custom display implementation.
#[derive(Debug, Default)]
struct TestCount(i64);

impl fmt::Display for TestCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            write!(f, "-")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Debug, Default)]
struct TestReport {
    /// Number of failed tests
    failed: TestCount,
    /// Number of passing tests
    passing: TestCount,
    /// Number of failing tests expected to fail
    xfail: TestCount,
    /// Number of passing tests expected to pass
    xpass: TestCount,
}

/// Structure representing the evolution of tests over a time period.
#[derive(Debug)]
pub struct TestCases {
    /// Old test results.
    previous: TestReport,
    /// New test results.
    current: TestReport,
}

/// Gather test results from the check-rust script
///
/// Build the project located at the given path and launch it's testsuite
/// then collect the test results.
///
/// * `repo_location` - The local repository location.
/// * `additional_args` - Additional arguments for the configure script.
fn gather_test_results(
    repo_location: &Path,
    additional_args: &str,
) -> Result<TestReport, ReportError> {
    let build_dir = repo_location.to_path_buf().join("build");
    // We do not want to mess with existing build
    if build_dir.exists() {
        panic!("A build directory already exists, please delete it.")
    }
    fs::create_dir(&build_dir).map_err(ReportError::LocalError)?;
    let mut configure_args = vec![
        "--enable-multilib",
        "--disable-bootstrap",
        "--enable-languages=rust",
    ];

    configure_args.push(additional_args);

    Command::new("../configure")
        .args(configure_args)
        .stdout(Stdio::null())
        .current_dir(&build_dir)
        .status()
        .map_err(ReportError::Configure)?;

    // TODO: We either want to use a computed value or leave this bit to the
    // user by providing an interface for additional make arguments rather than
    // hardcoding the job amount.
    let make_args = vec!["-j14"];

    Command::new("make")
        .args(make_args)
        .stdout(Stdio::null())
        .current_dir(&build_dir)
        .status()
        .map_err(ReportError::Make)?;

    let output = Command::new("make")
        .arg("check-rust")
        .current_dir(&build_dir)
        .output()
        .map_err(ReportError::Check)?;

    let s: Vec<&str> = std::str::from_utf8(&output.stdout)
        .map_err(ReportError::CollectionError)?
        .lines()
        .filter(|e| e.starts_with('#'))
        .collect();

    let xfail = s
        .iter()
        .find(|s| s.contains("failures"))
        .and_then(|s| s.split_whitespace().last())
        .and_then(|s| i64::from_str(s).ok())
        .map(TestCount)
        .unwrap_or_default();

    let passing = s
        .iter()
        .find(|s| s.contains("passes"))
        .and_then(|s| s.split_whitespace().last())
        .and_then(|s| i64::from_str(s).ok())
        .map(TestCount)
        .unwrap_or_default();

    let result = TestReport {
        xfail,
        passing,
        ..Default::default()
    };

    fs::remove_dir_all(build_dir).map_err(ReportError::LocalError)?;

    Ok(result)
}

impl TestCases {
    /// Collect the test reports.
    ///
    /// Build a project and collect the test reports for two different versions.
    ///
    /// # Arguments
    ///
    /// * `repo` - Git2 repository handle.
    /// * `previous` - Reference to the previous version.
    /// * `current` - Reference to the current version.
    /// * `configure_args` - Additional arguments to pass to the configure script.
    pub async fn collect(
        repo: &Repository,
        previous: &str,
        current: &str,
        configure_args: &str,
    ) -> Result<Self, ReportError> {
        use ReportError::CheckoutError;
        let workspace = repo
            .path()
            .parent()
            .expect(".git parent should always exist");
        repository::checkout(repo, previous).map_err(CheckoutError)?;

        let previous = gather_test_results(workspace, configure_args)?;

        repository::checkout(repo, current).map_err(CheckoutError)?;

        let current = gather_test_results(workspace, configure_args)?;

        Ok(TestCases { previous, current })
    }
}

impl fmt::Display for TestCases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const W_0: usize = 12;
        const W_1: usize = 10;
        const W_2: usize = 10;
        const W_3: usize = 5;
        const SEP: &str = "-";

        writeln!(
            f,
            "|{:<W_0$}|{:<W_1$}|{:<W_2$}|{:<W_3$}|",
            "Testcases", "Last week", "This week", "Delta",
        )?;
        writeln!(f, "|{SEP:->W_0$}+{SEP:->W_1$}+{SEP:->W_2$}+{SEP:->W_3$}|")?;
        writeln!(
            f,
            "|{:<W_0$}|{:<W_1$}|{:<W_2$}|{:<W_3$}|",
            "Passing",
            self.previous.passing,
            self.current.passing,
            TestCount(self.current.failed.0 - self.previous.failed.0),
        )?;

        writeln!(
            f,
            "|{:<W_0$}|{:<W_1$}|{:<W_2$}|{:<W_3$}|",
            "Failed",
            self.previous.failed,
            self.current.failed,
            TestCount(self.current.failed.0 - self.previous.failed.0),
        )?;

        writeln!(
            f,
            "|{:<W_0$}|{:<W_1$}|{:<W_2$}|{:<W_3$}|",
            "XFAIL",
            self.previous.xfail,
            self.current.xfail,
            TestCount(self.current.xfail.0 - self.previous.xfail.0),
        )?;

        writeln!(
            f,
            "|{:<W_0$}|{:<W_1$}|{:<W_2$}|{:<W_3$}|",
            "XPASS",
            self.previous.xpass,
            self.current.xpass,
            TestCount(self.current.xpass.0 - self.previous.xpass.0),
        )
    }
}
