use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct ReportError;

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error happened while reporting test cases")
    }
}

impl std::error::Error for ReportError {}

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
    failed: TestCount,
    passing: TestCount,
    xfail: TestCount,
    xpass: TestCount,
}

#[derive(Debug, Default)]
pub struct TestCases {
    previous: TestReport,
    current: TestReport,
}

impl TestCases {
    pub fn collect() -> Result<Self, ReportError> {
        Ok(TestCases::default())
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
