#![feature(extern_prelude)]
#[macro_use] extern crate structopt;
extern crate config;
extern crate serde;
#[macro_use] extern crate serde_derive;

use structopt::StructOpt;

mod options;
mod settings;

use options::Opts;
use settings::Settings;

fn main() {
    let opts = Opts::from_args();
    let app_settings = Settings::new(opts.config_file_path).unwrap();

    println!("{:?}", app_settings);

    println!("Database URL: {}", app_settings.database.url)
}
