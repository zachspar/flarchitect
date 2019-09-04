/* Flarchitect CLI
 *
 * Rapid development framework for python-flask apps.
 *
 * Author: Zachary Spar
 * Email : zachspar@gmail.com
 */
extern crate clap;

mod flarchitects;

use std::fs;
use std::path::PathBuf;
use clap::{Arg, App};
use flarchitects::{html_template_txt, view_template_txt,
                   init_template_txt, run_app_script_txt};
use std::io::Write;


fn main() {
    let matches = App::new("Flarchitect")
                        .version("0.1.0")
                        .author("Zachary Spar <zachspar@gmail.com>")
                        .about("Rapid development framework for python-flask apps")
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
                        .arg(Arg::with_name("view_name")
                                    .required(false)
                                    .short("v")
                                    .long("view_name")
                                    .takes_value(true)
                                    .help("view name"))
                        .arg(Arg::with_name("create_env")
                                    .required(false)
                                    .short("e")
                                    .long("create_env")
                                    .takes_value(false)
                                    .help("create a virtual enviroment for flask app"))
                        .arg(Arg::with_name("run_server")
                                    .required(false)
                                    .short("s")
                                    .long("run_server")
                                    .takes_value(true)
                                    .help("serve flask app on specified environment"))
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

            match create_server_script(project_name) {
                Ok(_) => println!("Created server script"),
                Err(err) => panic!("Error: could not create server script, {}", err),
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

        if matches.is_present("view_name") {
            let view_name = matches.value_of("view_name").unwrap();
            println!("View name: [{}]", view_name);
            match create_view(project_name, view_name) {
                Ok(_) => println!("Created view [{}] in project [{}]",
                                  view_name, project_name),
                Err(err) => panic!("Error: could not create view, {}", err),
            };
        }
    }

    if matches.is_present("create_env") {
        match create_venv() {
            Ok(_) => println!("Created virtual environment for project in dir [{}/env]",
                              get_cwd()),
            Err(err) => panic!("Error: could not create virtual environment, {}", err),
        };
    }

    if matches.is_present("run_server") {
        match run_server() {
            Ok(_) => println!("Running server..."),
            Err(err) => panic!("Error: could not run server, {}", err),
        };
    }
}


fn get_cwd() -> String {
    let cwd = std::env::current_dir().unwrap();
    let basename = cwd.into_os_string().into_string().unwrap();
    return basename;
}


fn run_server() -> std::io::Result<()> {
    println!("{}/bin/run_server.sh", get_cwd());
    std::process::Command::new(format!("{}/bin/run_server.sh", get_cwd()))
                          .output()
                          .expect(&format!("failed to start server script...{}",
                                  get_cwd()));
    Ok(())
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
            Ok(_) => println!("Created dir [{}/{}/{}]", basename, p_name, dir),
            Err(err) => return Err(err),
        };
    }

    let mut file = fs::File::create(format!("{}/{}/__init__.py", get_cwd(), p_name))?;
    file.write_all(init_template_txt(p_name, "root").as_bytes())?;

    let mut file = fs::File::create(format!("{}/{}/views/__init__.py", get_cwd(), p_name))?;
    file.write_all(init_template_txt(p_name, "view").as_bytes())?;

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


fn create_view(p_name: &str, v_name: &str) -> std::io::Result<()> {
    println!("Creating view [{}] in project [{}]", v_name, p_name);
    let basename = get_cwd();
    // add this new view to pre-existing init file
    let init_file_string  = format!("{}/{}/views/__init__.py", basename, p_name);
    let mut init_file_handle = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(init_file_string)
        .unwrap();

    if let Err(e) = writeln!(init_file_handle, "from {}.views.{} import *", p_name, v_name) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let file_path = PathBuf::from(format!("{}/{}/{}/{}.py",
                                          basename, p_name, "views", v_name));
    let mut file = fs::File::create(file_path)?;
    file.write_all(view_template_txt(p_name, v_name).as_bytes())?;
    Ok(())
}


fn create_venv() -> std::io::Result<()> {
    std::process::Command::new("python3")
                          .args(&["-m", "venv", "env"])
                          .output()
                          .expect("failed to create virtual environment...");
    std::process::Command::new(&format!(". ./env/bin/activate"));
    Ok(())
}


fn create_server_script(project_name: &str) -> std::io::Result<()> {
    /*
    let path = format!("{}/{}/run_server.sh", get_cwd(), "bin");
    let mut file = OpenOptions::new()
                                   .create(true)
                                   .write(true)
                                   .mode(0o770)
                                   .open(&path)
                                   .unwrap();
                                   */

    let file_path = PathBuf::from(format!("{}/{}/run_server.sh", get_cwd(), "bin"));
    let mut file = fs::File::create(file_path)?;
    file.write_all(run_app_script_txt(project_name, &get_cwd()).as_bytes())?;
    Ok(())
}

