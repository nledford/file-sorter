use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Nathaniel Ledford <nate@nateledford.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    Sort(Sort),
    Append(Append),
}

#[derive(Parser)]
pub struct Sort {
    pub dir: Option<String>,
}

#[derive(Parser)]
pub struct Append {
    pub dir: Option<String>,
    // Append randomly generated dates rather than the date from the file itself
    #[clap(long, takes_value = false)]
    pub random_dates: bool,
}
