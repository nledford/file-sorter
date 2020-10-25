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
    pub dir: String,
}

#[derive(Clap)]
pub struct Append {
    pub dir: String,
}
