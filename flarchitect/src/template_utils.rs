/*
    This is the template utilities module for Flarchitect.

    It contains all of the utility functions involved with
    creating templates.

    TODO : install custom html template "base class"

    Author: Zachary Spar <zachspar@gmail.com>
*/
use std::io::Write;
use crate::flarc_utils::{get_cwd, html_template_txt};


pub(crate) fn create_html_template(p_name: &str, t_name: &str) -> std::io::Result<String> {
    println!("Creating HTML template [{}] in project [{}]", t_name, p_name);
    let basename = get_cwd();
    let file_path = std::path::PathBuf::from(format!("{}/{}/{}/{}.html",
                                          basename, p_name, "templates", t_name));
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(html_template_txt(p_name).as_bytes())?;
    Ok(String::from(format!("Created template [{}] in project [{}]", t_name, p_name)))
}
