use std::env::args;
use std::fs::{create_dir_all, write};
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        eprintln!("Not enough arguments. Use {} -h for help.", args[0]);
        exit(0);
    }
    let mut dir_name: &str = "";
    for arg in &args {
        match &arg[..] {
            "-h" => show_help(&args[0]),
            "-v" => version_info(),
            name @ _ => dir_name = name,
        }
    }
    let src_dir = dir_name.to_owned() + "/src";
    let obj_dir = src_dir.clone() + "/obj";
    let inc_dir = dir_name.to_owned() + "/include";
    let root_path = Path::new(dir_name);
    let src_path = Path::new(&src_dir);
    let inc_path = Path::new(&inc_dir);
    let obj_path = Path::new(&obj_dir);

    let paths = vec![root_path, src_path, inc_path, obj_path];
    build_dirs(&paths);
    populate_dirs(&paths);
    git_init(&paths);

    println!("Created new template at {}", paths[0].to_str().unwrap());
}

fn build_dirs(paths: &Vec<&Path>) {
    create_dir_all(paths[1]).expect("Error creating src dir");
    create_dir_all(paths[2]).expect("Error creating inc dir");
    create_dir_all(paths[3]).expect("Error creating obj dir");
}

fn populate_dirs(paths: &Vec<&Path>) {
    let makefile = "\
CC=gcc
# .c files here
SRCDIR=./src
# .h files here
IDIR=./include
# compiled .o files here
ODIR=./src/obj

CFLAGS=-I$(IDIR) -Wall -std=c11

# Header files here
_DEPS=
DEPS=$(patsubst %,$(IDIR)/%,$(_DEPS))

# Object files here
_OBJ=main.o
OBJ=$(patsubst %,$(ODIR)/%,$(_OBJ))

# Required libraries here
LIBS=

main: $(OBJ)
\t$(CC) -o $@ $^ $(CFLAGS) $(LIBS)

$(ODIR)/%.o: $(SRCDIR)/%.c $(DEPS)
\t$(CC) -c -o $@ $< $(CFLAGS)

.PHONY: clean

clean:
\trm -f *.exe $(ODIR)/*.o";

    let main = "\
#include <stdio.h>

int main(int argc, char *argv[])
{
    printf(\"Hello world\\n\");
    return(0);
}";
    let gitignore = "\
*.exe
*.o";
    let make_path = paths[0].to_str().unwrap().to_owned() + "/Makefile";
    let main_path = paths[1].to_str().unwrap().to_owned() + "/main.c";
    let gitignore_path = paths[0].to_str().unwrap().to_owned() + "/.gitignore";
    write(make_path, makefile).expect("Error writing Makefile");
    write(main_path, main).expect("Error writing main.c");
    write(gitignore_path, gitignore).expect("Error writing .gitignore");
}

fn git_init(paths: &Vec<&Path>) {
    let result = Command::new("git")
        .arg("init")
        .arg(paths[0].to_str().unwrap())
        .output()
        .expect("Error initializing git repo");
    println!("{}", String::from_utf8_lossy(&result.stdout));
}

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
    println!("cnew v0.1.0 by makos <Mateusz Makowski>, licensed under the MIT license.");
    exit(0);
}
