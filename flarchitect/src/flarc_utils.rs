/*
    This is the "flarc" utilities module for Flarchitect.

    It contains all of the utility functions involved with
    creating static templates, views, and general tool utils.

    Author: Zachary Spar <zachspar@gmail.com>
*/
use std::io::Write;


pub(crate) fn html_template_txt(app_name: &str) -> String {
    let ret_string = format!("<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n{}",
                             app_name, "<body>{{greeting}}</body>\n</html>");
    String::from(ret_string)
}


pub(crate) fn view_template_txt(app_name: &str, view_name: &str) -> String {
    let ret_string = format!("from flask import render_template\n\
                              import {}\n@{}.app.route('/{}/<string:name>', methods=[\"GET\", \"POST\"])\n\
                              def {}(name):\n    context = {{'greeting': 'Hello {{}}'.format(name)}}\
                              \n    return render_template('{}.html', **context)\n",
                             app_name, app_name, view_name, view_name, view_name);
    String::from(ret_string)
}


pub(crate) fn init_template_txt(app_name: &str, tmp_type: &str) -> String {

    let mut ret_string: String = "".to_string();
    if tmp_type == "root" {
        ret_string = String::from(format!("import flask\napp = flask.Flask(__name__)\
                                  \nimport {}\nimport {}.views\n", app_name, app_name));
    }
    else if tmp_type == "view" {
        ret_string = String::from(format!("from {}.views import *\n", app_name));
    }

    assert_ne!(ret_string, String::from(""));
    ret_string
}


pub(crate) fn run_app_script_txt(app_name: &str, cwd: &String) -> String {
    let ret_string = format!("#!/bin/bash\nsource {}/env/bin/activate;\
                             export LC_ALL=en_US.UTF-8;export LANG=en_US.UTF-8;\
                             export FLASK_DEBUG=True;export FLASK_APP={}/{};flask run",
                             cwd, cwd, app_name);
    String::from(ret_string)
}


pub(crate) fn setup_app_txt(app_name: &str) -> String {
    let spaces = "    ";
    String::from(format!("from setuptools import setup\n\
    \nsetup(\n{}name=\"{}\",\n{}version=\"1.0\",\n{}long_description=__doc__,\n{}packages=[\"{}\"],\
    \n{}include_package_data=True,\n{}zip_safe=False,\n{}install_requires=['Flask',]\n)\n\n",
                         spaces, app_name, spaces, spaces, spaces, app_name, spaces, spaces, spaces))
}


pub(crate) fn get_cwd() -> String {
    let cwd = std::env::current_dir().unwrap();
    let basename = cwd.into_os_string().into_string().unwrap();
    return basename;
}


// TODO : need to somehow change the mode of the server script to be exec
pub(crate) fn create_server_script(project_name: &str) -> std::io::Result<String> {
    let server_filename = format!("{}/{}/run_server.sh", get_cwd(), "bin");
    let file_path = std::path::PathBuf::from(&server_filename);
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(run_app_script_txt(project_name, &get_cwd()).as_bytes())?;
    Ok(String::from("Created run server script"))
}


pub(crate) fn create_venv() -> std::io::Result<String> {
    std::process::Command::new("python3")
        .args(&["-m", "venv", "env"])
        .output()
        .expect("failed to create virtual environment...");
    std::process::Command::new(&format!(". ./env/bin/activate"));
    std::process::Command::new("pip install -U pip; pip install -e .");
    Ok(String::from(format!("Created virtual environment env in directory [ {} ]", get_cwd())))
}


pub(crate) fn create_project_archetype(p_name: &str) -> std::io::Result<String> {
    println!("Creating project architecture for: [{}]", p_name);
    let dirs = vec!["views", "templates", "static"];
    let basename = get_cwd();

    match std::fs::create_dir_all(format!("{}/{}", basename, "bin")) {
        Ok(_) => println!("Created dir: [{}/{}", basename, "bin"),
        Err(err) => return Err(err),
    };

    for dir in dirs {
        match std::fs::create_dir_all(format!("{}/{}/{}", basename, p_name, dir)) {
            Ok(_) => println!("Created dir [{}/{}/{}]", basename, p_name, dir),
            Err(err) => return Err(err),
        };
    }

    let mut file = std::fs::File::create(format!("{}/{}/__init__.py", get_cwd(), p_name))?;
    file.write_all(init_template_txt(p_name, "root").as_bytes())?;
    println!("create_project_archetype :: Created __init__ for root project");

    let mut file = std::fs::File::create(format!("{}/{}/views/__init__.py", get_cwd(), p_name))?;
    file.write_all(init_template_txt(p_name, "view").as_bytes())?;
    println!("create_project_archetype :: Created __init__ for views");

    let mut file = std::fs::File::create(format!("{}/setup.py", get_cwd()))?;
    file.write_all(setup_app_txt(p_name).as_bytes())?;
    println!("create_project_archetype :: Created setup.py file for project");

    Ok(String::from(format!("Created project architecture for [{}]", p_name)))
}