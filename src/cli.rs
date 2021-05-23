use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cli {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Clap, Debug)]
pub enum Cmd {
    New(New),
    Init(Init),
}

#[derive(Clap, Debug)]
pub struct New {
    name: String,
}

#[derive(Clap, Debug)]
pub struct Init {
    name: Option<String>,
}
