use clap::Clap;

mod cli;
use cli::*;
use std::{env, fs, io::ErrorKind};
use std::{fs::File, io::Write};

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Cmd::Init(Init { name, .. }) => {
            File::create("CMakeLists.txt")
                .map(|file| base_cmake(file, name))
                .expect("Could not create file");
        }
        Cmd::New(New { name, .. }) => {
            if let Err(error) = fs::create_dir(name) {
                if error.kind() == ErrorKind::AlreadyExists {
                    println!("Directory with name \"{}\" already exists.", name);
                    std::process::exit(1);
                }
                panic!("Error: {:?}", error.kind());
            }

            File::create(format!("{}/CMakeLists.txt", name))
                .map(|file| base_cmake(file, &Some(name.to_owned())))
                .expect("Could not create file");
        }
    }

    println!("{:?}", cli);
}

fn base_cmake(mut file: File, name: &Option<String>) {
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
    .expect("Could not write to file");
}
