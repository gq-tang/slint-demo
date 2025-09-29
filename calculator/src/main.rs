use anyhow::Result;

slint::include_modules!();

fn main() -> Result<()> {
    let app = MainWindow::new()?;

    let app_weak = app.as_weak();

    app.on_keyPressed(move |key| {
        if let Some(app) = app_weak.upgrade() {
            let current_text = app.get_exp();
            let new_text = match key.as_str() {
                "+" | "-" | "*" | "/" => {
                    if current_text.is_empty() || current_text == "0" {
                        slint::SharedString::from("0")
                    } else if current_text.ends_with("+")
                        || current_text.ends_with("-")
                        || current_text.ends_with("*")
                        || current_text.ends_with("/")
                    {
                        slint::SharedString::from(format!(
                            "{}{}",
                            &current_text[0..current_text.len() - 1],
                            key
                        ))
                    } else if current_text.ends_with(".") {
                        return;
                    } else {
                        slint::SharedString::from(format!("{}{}", current_text, key))
                    }
                }
                "." => {
                    if current_text.is_empty() || current_text == "0" {
                        slint::SharedString::from("0.")
                    } else if current_text.ends_with(".") {
                        return;
                    } else if current_text.ends_with("+")
                        || current_text.ends_with("-")
                        || current_text.ends_with("*")
                        || current_text.ends_with("/")
                    {
                        slint::SharedString::from(format!("{}0.", current_text))
                    } else {
                        slint::SharedString::from(format!("{}{}", current_text, key))
                    }
                }
                _ => {
                    if current_text.is_empty() || current_text == "0" {
                        slint::SharedString::from(key)
                    } else {
                        slint::SharedString::from(format!("{}{}", current_text, key))
                    }
                }
            };

            app.set_exp(new_text);
        }
    });

    let app_weak = app.as_weak();
    app.on_delete(move || {
        if let Some(app) = app_weak.upgrade() {
            let current_text = app.get_exp();
            let new_text = match current_text.len() {
                0 => slint::SharedString::from("0"),
                1 => slint::SharedString::from("0"),
                _ => slint::SharedString::from(&current_text[0..current_text.len() - 1]),
            };
            app.set_exp(new_text);
        }
    });

    let app_weak = app.as_weak();
    app.on_calculate(move || {
        if let Some(app) = app_weak.upgrade() {
            let current_text = app.get_exp();
            if current_text.ends_with("+")
                || current_text.ends_with("-")
                || current_text.ends_with("*")
                || current_text.ends_with("/")
                || current_text.ends_with(".")
            {
                return;
            }
            let res = match meval::eval_str(&current_text) {
                Ok(reult) => reult.to_string(),
                Err(e) => e.to_string(),
            };
            let new_text = slint::SharedString::from(format!("{}\n= {}", current_text, res));
            app.set_result(new_text);
        }
    });
    app.run()?;
    Ok(())
}
