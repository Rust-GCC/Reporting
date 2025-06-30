use std::fmt;

pub struct Task
{
     pub nb_todo:u64,
     pub in_prog:u64,
     pub closed:u64,
}

impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const W_0: usize = 12;
        const W_1: usize = 10;
        const W_2: usize = 10;
        const W_3: usize = 5;
        const SEP: &str = "-";

        writeln!(
            f,
            "|{:<W_0$}|{:<W_1$}|{:<W_2$}|{:<W_3$}|",
            "Category", "Last week", "This week", "Delta",
        )?;
        writeln!(f, "|{SEP:->W_0$}+{SEP:->W_1$}+{SEP:->W_2$}+{SEP:->W_3$}|")?;
        writeln!(
            f,
            "|{:>W_0$}|{:>W_1$}|{:>W_2$}|{:>W_3$}|",
            "TODO",
            format!("-"),
            format!("{}", self.nb_todo),
            format!("-"),
        )?;

        writeln!(
            f,
            "|{:>W_0$}|{:>W_1$}|{:>W_2$}|{:>W_3$}|",
            "In Progress",
            format!("-"),
            format!("{}", self.in_prog),
            format!("-"),
        )?;

        writeln!(
            f,
            "|{:>W_0$}|{:>W_1$}|{:>W_2$}|{:>W_3$}|",
            "Completed",
            format!("-"),
            format!("{}", self.closed),
            format!("-"),
        )
   
    }
}

