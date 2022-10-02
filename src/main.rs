use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{Browser, Tab};
use std::error::Error;
use std::fs;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    let mut url = String::from("http://10.25.168.50");
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(&url)?;
    tab.wait_until_navigated().unwrap();
    let username_input = tab.wait_for_element("#Frm_Username")?;
    let password_input = tab.wait_for_element("#Frm_Password")?;
    let username_remote = username_input.call_js_fn(
        r#"
    function set_username () {
        this.value = "admin"
        return this.value;
    }
"#,
        vec![],
        false,
    )?;
    match username_remote.value {
        Some(returned_string) => {
            println!("{}", &returned_string);
        }
        _ => unreachable!(),
    };

    let password_remote = password_input.call_js_fn(
        r#"
    function set_password () {
        this.value = "admin"
        return this.value;
    }
"#,
        vec![],
        false,
    )?;
    match password_remote.value {
        Some(returned_string) => {
            println!("{}", &returned_string);
        }
        _ => unreachable!(),
    };

    tab.press_key("Enter")?;
    tab.wait_for_element("body")?;
    url.push_str("/template.gch?pid=1002&nextpage=manager_dev_conf_t.gch");
    tab.navigate_to(&url)?;
    match tab
        .wait_for_element("body")?
        .call_js_fn(
            r#"
    function reboot () {
        remove_msgbox();msgCallback();
        return true;
    }
"#,
            vec![],
            false,
        )?
        .value
    {
        Some(_retured_value) => {
            println!("Router is rebooting.")
        }
        _ => {
            println!("Failed to reboot.")
        }
    }
    screenshot(&tab)?;

    Ok(())
}

fn screenshot(tab: &Arc<Tab>) -> Result<(), Box<dyn Error>> {
    let png_data = tab
        .wait_for_element("body")?
        .capture_screenshot(CaptureScreenshotFormatOption::Png)?;
    fs::write("screenshot.png", &png_data)?;
    println!("Screenshots successfully created.");
    Ok(())
}
