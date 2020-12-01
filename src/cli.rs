use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Nathaniel Ledford <nate@nateledford.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Sort(Sort),
    Append(Append),
}

#[derive(Clap)]
pub struct Sort {
    pub dir: Option<String>,
}

#[derive(Clap)]
pub struct Append {
    pub dir: Option<String>,
    // Append randomly generated dates rather than the date from the file itself
    #[clap(long, takes_value = false)]
    pub random_dates: bool,
}
