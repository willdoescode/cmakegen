use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(Clap, Debug)]
pub enum Cmd {
    New(New),
    Init(Init),
}

#[derive(Clap, Debug)]
pub struct New {
    pub name: String,
}

#[derive(Clap, Debug)]
pub struct Init {
    pub name: Option<String>,
}
