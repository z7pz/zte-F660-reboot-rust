use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{Browser, Tab};
use serde_json::value::Value;
use std::error::Error;
use std::fs;
use std::sync::Arc;
fn main() -> Result<(), Box<dyn Error>> {
    let mut url = String::from("http://10.25.168.50");
    let username = String::from("admin");
    let password = String::from("admin");

    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to(&url)
        .unwrap_or_else(|_| panic!("couldn't navigate to \"{}\"!", &url));
    tab.wait_until_navigated()
        .unwrap_or_else(|_| panic!("couldn't navigate to \"{}\"!", &url));
    let username_input = tab.wait_for_element("#Frm_Username").unwrap();
    let password_input = tab.wait_for_element("#Frm_Password").unwrap();
    let username_remote = username_input
        .call_js_fn(
            r#"function set_username (username) {this.value = username;return this.value;}"#,
            vec![Value::String(username)],
            false,
        )
        .unwrap();
    match username_remote.value {
        Some(_returned_string) => println!("✔ username"),
        _ => unreachable!(),
    };

    let password_remote = password_input
        .call_js_fn(
            r#"function set_password (password) {this.value = password;return this.value;}"#,
            vec![Value::String(password)],
            false,
        )
        .unwrap();
    match password_remote.value {
        Some(_returned_string) => println!("✔ password"),
        _ => unreachable!(),
    };

    tab.press_key("Enter").unwrap();
    tab.wait_for_element("body").unwrap();
    url.push_str("/template.gch?pid=1002&nextpage=manager_dev_conf_t.gch");
    tab.navigate_to(&url)
        .unwrap_or_else(|_| panic!("couldn't navigate to \"{}\"!", &url));
    match tab
        .wait_for_element("body")
        .unwrap()
        .call_js_fn(
            r#"function reboot () {remove_msgbox();msgCallback();return true;}"#,
            vec![],
            false,
        )
        .unwrap()
        .value
    {
        Some(_retured_value) => println!("🐱‍🏍 Router is rebooting."),
        _ => panic!("Failed to reboot."),
    }
    screenshot(&tab).unwrap_or_else(|_| panic!("couldn't take a screenshot!"));

    Ok(())
}

fn screenshot(tab: &Arc<Tab>) -> Result<(), Box<dyn Error>> {
    let png_data = tab
        .wait_for_element("body")
        .expect("couldn't find body element")
        .capture_screenshot(CaptureScreenshotFormatOption::Png)
        .expect("couldn't take a screenshot");
    fs::write("screenshot.png", &png_data)
        .unwrap_or_else(|_| panic!("couldn't take a screenshot!"));
    Ok(())
}
