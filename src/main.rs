use std::{num::NonZero, process::Command};

use clap::Parser;

#[derive(Debug, Parser)]
struct App {
    #[clap(short, long)]
    jobs: Option<NonZero<usize>>,
}

fn main() {
    let app = App::parse();
    let jobs = app
        .jobs
        .unwrap_or_else(|| std::thread::available_parallelism().unwrap());

    println!("spinning up {} jobs", jobs);

    loop {
        let mut handles = Vec::new();

        for n in 0..jobs.get() {
            let handle = std::thread::spawn(move || {
                println!("*** job {} started", n);
                run_cargo_build();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("thread join success");
        }

        std::fs::remove_dir_all("nextest-tests/target").expect("remove target directory success");
    }

    // let mut handles = Vec::new();

    // for n in 0..jobs.get() {
    //     let handle = std::thread::spawn(move || {
    //         loop {
    //             println!("*** job {} started", n);
    //             run_cargo_build();
    //         }
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().expect("thread join success");
    // }

    // std::fs::remove_dir_all("nextest-tests/target").expect("remove target directory success");
}

fn run_cargo_build() {
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--workspace")
        .current_dir("nextest-tests");
    let mut child = command.spawn().expect("command spawn success");
    child.wait().expect("command wait success");
}
