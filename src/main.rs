extern crate clap;

use std::fs;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Flarchitect")
        .version("0.1.0")
        .author("Zachary Spar <zachspar@gmail.com>")
        .about("Rapid python-flask app development")
        .arg(Arg::with_name("project_name")
                    .required(true)
                    .short("p")
                    .long("project_name")
                    .takes_value(true)
                    .help("your projects name"))
        .get_matches();
    let project_name = matches
                        .value_of("project_name")
                        .unwrap();
    println!("Project name:  [{}]", project_name);
    create_project_archetype(project_name);
}


fn create_project_archetype(p_name: &str) -> std::io::Result<()> {
    println!("Creating project architecture for: [{}]", p_name);
    let dirs = vec!["views", "templates", "static"];
    let cwd = std::env::current_dir().unwrap();
    let basename = cwd.into_os_string().into_string().unwrap();

    fs::create_dir_all(format!("{}/{}",
                               basename, "bin"));

    for dir in dirs {
        println!("Creating dir [{}/{}/{}]", basename, p_name, dir);
        fs::create_dir_all(format!("{}/{}/{}",
                                   basename, p_name, dir));
    }
    Ok(())
}

