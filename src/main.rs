use clap::Clap;

mod cli;
use cli::*;
use std::env;
use std::{fs::File, io::Write};

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Cmd::Init(Init { name, .. }) => {
            File::create("CMakeLists.txt")
                .and_then(|mut file| {
                    file.write_all(
                        format!(
                            "cmake_minimum_required(VERSION 3.20.2)\nproject({} C)",
                            name.as_ref().unwrap_or(
                                &env::current_dir()
                                    .expect("Could not get working dir")
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string()
                            )
                        )
                        .as_bytes(),
                    )
                })
                .expect("Could not create file");
        }
        Cmd::New(New { name, .. }) => println!("New: {}", name),
    }

    println!("{:?}", cli);
}
