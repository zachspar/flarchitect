/*
    This is the view utilities module for Flarchitect.

    It contains all of the utility functions involved with
    creating views.

    Author: Zachary Spar <zachspar@gmail.com>
*/
use std::io::Write;
use crate::flarc_utils::{get_cwd, view_template_txt};


pub(crate) fn create_view(p_name: &str, v_name: &str) -> std::io::Result<String> {
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

    let file_path = std::path::PathBuf::from(format!("{}/{}/{}/{}.py",
                                          basename, p_name, "views", v_name));
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(view_template_txt(p_name, v_name).as_bytes())?;
    Ok(String::from(format!("Created view [{}] in project [{}]", v_name, p_name)))
}
