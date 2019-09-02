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

pub fn init_template_txt(app_name: &str) -> String {
    let ret_string = format!("import flask\napp = flask.Flask(__name__)\
                             \napp.config.from_object('{}.config')\
                             \nimport {}\nimport {}.views\nimport {}.config\n",
                             app_name, app_name, app_name, app_name);
    return String::from(ret_string);
}

