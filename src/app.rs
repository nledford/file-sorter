use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Nathaniel Ledford <nate@nateledford.com>")]
pub struct Opts {
    /// The directory of files to be organized
    pub dir: String,
}
