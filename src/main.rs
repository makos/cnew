use std::env::args;
use std::fs;
use std::io;
use std::path;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        eprintln!("Not enough arguments. Use {} -h for help.", args[0]);
        exit(0);
    }
    let mut dir_name: String = String::new();
    // let mut dir_path: path::Path::new(".");
    for arg in &args {
        match &arg[..] {
            "-h" => show_help(&args[0]),
            "-v" => version_info(),
            name @ _ => dir_name = name.to_owned(),
        }
    }
    build_dirs(&dir_name);
    populate_dirs();
    git_init();
}

fn build_dirs(dir_name: &String) {
    let mut src_dir = dir_name.clone();
    let mut inc_dir = dir_name.clone();
    src_dir.push_str("/src");
    inc_dir.push_str("/include");
    let src_path = path::Path::new(&src_dir);
    let inc_path = path::Path::new(&inc_dir);
    fs::create_dir_all(src_path).expect("Error creating src dir");
    fs::create_dir_all(inc_path).expect("Error creating inc dir");
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
