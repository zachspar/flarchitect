pub fn html_template_txt(app_name: &str) -> String {
    let ret_string = format!("<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n{}",
                             app_name, "<body>{{greeting}}</body>\n</html>");
    return String::from(ret_string);
}

