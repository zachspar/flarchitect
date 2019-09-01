extern crate clap;

mod flarchitects;

use std::fs;
use std::path::PathBuf;
use clap::{Arg, App};
use flarchitects::html_template_txt;
use std::io::Write;

fn main() {
    let matches = App::new("Flarchitect")
        .version("0.1.0")
        .author("Zachary Spar <zachspar@gmail.com>")
        .about("Rapid python-flask app development")
        .arg(Arg::with_name("project_name")
                    .required(false)
                    .short("p")
                    .long("project_name")
                    .takes_value(true)
                    .help("project name"))
        .arg(Arg::with_name("template_name")
                    .required(false)
                    .short("t")
                    .long("template_name")
                    .takes_value(true)
                    .help("template name"))
        .get_matches();

    if matches.is_present("project_name") {
        let project_name = matches
                            .value_of("project_name")
                            .unwrap();
        println!("Project name:  [{}]", project_name);

        if !std::path::Path::new(&format!("{}/{}", get_cwd(), project_name)).exists() {

            match create_project_archetype(project_name) {
                Ok(_) => println!("Created project archetecture!"),
                Err(err) => panic!("Error: could not create project architecture, {:?}", err),
            };
        }

        if matches.is_present("template_name") {
            let template_name = matches.value_of("template_name").unwrap();
            println!("Template name: [{}]", template_name);
            match create_html_template(project_name, template_name) {
                Ok(_) => println!("Created template [{}] in project [{}]",
                                  template_name, project_name),
                Err(err) => panic!("Error: could not create template, [{}]",
                                   err),
            };
        }
    }
}


fn get_cwd() -> String {
    let cwd = std::env::current_dir().unwrap();
    let basename = cwd.into_os_string().into_string().unwrap();
    return basename;
}


fn create_project_archetype(p_name: &str) -> std::io::Result<()> {
    println!("Creating project architecture for: [{}]", p_name);
    let dirs = vec!["views", "templates", "static"];
    let basename = get_cwd();

    match fs::create_dir_all(format!("{}/{}", basename, "bin")) {
        Ok(_) => println!("Created dir: [{}/{}", basename, "bin"),
        Err(err) => return Err(err),
    };

    for dir in dirs {
        match fs::create_dir_all(format!("{}/{}/{}", basename, p_name, dir)) {
            Ok(_) => println!("Creating dir [{}/{}/{}]", basename, p_name, dir),
            Err(err) => return Err(err),
        };
    }

    Ok(())
}


fn create_html_template(p_name: &str, t_name: &str) -> std::io::Result<()> {
    println!("Creating HTML template [{}] in project [{}]", t_name, p_name);
    let basename = get_cwd();
    let file_path = PathBuf::from(format!("{}/{}/{}/{}.html",
                                          basename, p_name, "templates", t_name));
    let mut file = fs::File::create(file_path)?;
    file.write_all(html_template_txt(p_name).as_bytes())?;

    Ok(())
}

