/* Flarchitect CLI
 *
 * Rapid development framework for python-flask apps.
 TODO : add .gitignore flarc util !!!
 TODO : add a dot file which will serve as configurations for Flarchitect
 ******* NOTE TODO THIS INVOLVES CREATING A FUCKING ENV VARIABLE OF PROJECT ROOT MORON!!!!!!
 *
 * Author: Zachary Spar
 * Email : zachspar@gmail.com
 */
extern crate clap;

mod view_utils;
mod flarc_utils;
mod template_utils;

use clap::{Arg, App};
use view_utils::{create_view};
use template_utils::{create_html_template};
use flarc_utils::{get_cwd, create_venv, create_server_script, create_project_archetype};
//use crate::flarc_utils::create_gitignore; // DANGEROUS AF


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
        .arg(Arg::with_name("template_name") // TODO : add subcommand custom input template
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
            .help("create a virtual environment for flask app"))
        .arg(Arg::with_name("run_server")
            .required(false)
            .short("s")
            .long("run_server")
            .takes_value(true)
            .help("serve flask app on specified environment"))
        .get_matches();

    if let Some(project_name) = matches.value_of("project_name") {

        if ! std::path::Path::new(&format!("{}/{}", get_cwd(), project_name)).exists() {

            match create_project_archetype(project_name) {
                Ok(msg) => println!("{}", msg),
                Err(err) => panic!("ERROR: could not create project architecture, {:?}", err),
            };

            // TODO : this is a VERY dangerous thing, as it overwrites my project gitignore lol :(
//            match create_gitignore(project_name) {
//                Ok(msg) => println!("{}", msg),
//                Err(err) => println!("ERROR: could not create .gitignore file, {:?}", err),
//            }

            match create_server_script(project_name) {
                Ok(msg) => println!("{}", msg),
                Err(err) => println!("ERROR: could not create server script, {:?}", err),
            };

        }

        // TODO : will incorporate subcommand to load new template name into system
        // --> this also will require finding patterns within templates
        if let Some(template_name) = matches.value_of("template_name") {
            match create_html_template(project_name, template_name) {
                Ok(msg) => println!("{}", msg),
                Err(err) => println!("ERROR: could not create HTML template, {:?}", err),
            };
        }

        // TODO : see above for template param... needs to match specification
        if let Some(view_name) = matches.value_of("view_name") {
            match create_view(project_name, view_name) {
                Ok(msg) => println!("{}", msg),
                Err(err) => println!("ERROR: could not create view, {:?}", err),
            };
        }
    }

    if matches.is_present("create_env") {
        match create_venv() {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("ERROR: could not create virtual environment, {:?}", err),
        };
    }
    // TODO : print uage command
    // if zero arguments are provided, print usage
}
