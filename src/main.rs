use image_sorter::app::Opts;
use clap::Clap;

fn main() {
    let opts = Opts::parse();

    let dir: String = opts.dir;
    println!("DIR:{}", &dir);
}
