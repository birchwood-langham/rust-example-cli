#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example cli application that accepts command line parameters and takes settings from config files", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opts {
    #[structopt(short = "c", long = "config")]
    pub config_file_path: Option<String>,
}
