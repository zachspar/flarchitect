pub fn html_template_txt(app_name: &str) -> String {
    let ret_string = format!("<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n{}",
                             app_name, "<body>{{greeting}}</body>\n</html>");
    return String::from(ret_string);
}


pub fn view_template_txt(app_name: &str, view_name: &str) -> String {
    let ret_string = format!("from flask import render_template\n\
                              import {}\n@{}.app.route('/{}/<string:name>', methods=[\"GET\", \"POST\"])\n\
                              def {}(name):\n    context = {{'greeting': 'Hello {{}}'.format(name)}}\
                              \n    return render_template('{}.html', **context)\n",
                              app_name, app_name, view_name, view_name, view_name);
    return String::from(ret_string);
}


pub fn init_template_txt(app_name: &str, tmp_type: &str) -> String {

    let mut ret_string: String = "".to_string();
    if tmp_type == "root" {
        ret_string = String::from(format!("import flask\napp = flask.Flask(__name__)\
                                  \nimport {}\nimport {}.views\n", app_name, app_name));
    }
    else if tmp_type == "view" {
        ret_string = String::from(format!("from {}.views import *\n", app_name));
    }

    assert!(ret_string != "");
    return ret_string;
}


pub fn run_app_script_txt(app_name: &str, cwd: &String) -> String {
    let ret_string = format!("#!/bin/bash\nsource {}/env/bin/activate;\
                             export LC_ALL=en_US.UTF-8;export LANG=en_US.UTF-8;\
                             export FLASK_DEBUG=True;export FLASK_APP={}/{};flask run",
                             cwd, cwd, app_name);
    return String::from(ret_string);
}

