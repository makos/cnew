use std::env::args;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        eprintln!("Not enough arguments. Use {} -h for help.", args[0]);
        exit(0);
    }
    let mut dir_name: &str = "";
    // let mut dir_path: path::Path::new(".");
    for arg in &args {
        match &arg[..] {
            "-h" => show_help(&args[0]),
            "-v" => version_info(),
            name @ _ => dir_name = name,
        }
    }
    let src_dir = dir_name.to_owned() + "/src";
    let inc_dir = dir_name.to_owned() + "/include";
    let src_path = Path::new(&src_dir);
    let inc_path = Path::new(&inc_dir);

    let paths = vec![src_path, inc_path];
    build_dirs(&paths);
    populate_dirs();
    git_init();
}

fn build_dirs(paths: &Vec<&Path>) {
    fs::create_dir_all(paths[0]).expect("Error creating src dir");
    fs::create_dir_all(paths[1]).expect("Error creating inc dir");
}

fn populate_dirs() {}

fn git_init() {}

fn show_help(bin_name: &String) {
    println!(
        "{} - create simple C directory template for rapid development",
        bin_name
    );
    println!("Usage: {} [OPTIONS] NAME", bin_name);
    println!("Where OPTIONS are:");
    println!("-h -- show this help");
    println!("-v -- show version info");
    exit(0);
}

fn version_info() {
    println!("cnew v0.1.0");
    exit(0);
}
