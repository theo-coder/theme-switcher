use std::{
    fs::{self, OpenOptions},
    io::Write,
};

const LIGHT_THEME: &str = "Arc";
const DARK_THEME: &str = "Arc-Dark";

fn main() -> Result<(), systray::Error> {
    let mut app;
    let file_path = format!(
        "{}/.xsettingsd",
        dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    );

    let file_content = fs::read_to_string(file_path).expect("Failed to read xsettings file");

    let file_theme = file_content.split("\"").collect::<Vec<&str>>()[1];
    let mut theme: String;

    let icon_file = if file_theme.to_lowercase().contains("dark") {
        theme = String::from("dark");
        "./assets/moon.svg"
    } else {
        theme = String::from("light");
        "./assets/sun.svg"
    };

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_file(icon_file)?;

    app.add_menu_item("Change theme", move |app| {
        println!("Changing theme");

        match theme.as_str() {
            "dark" => {
                app.set_icon_from_file("./assets/sun.svg")?;

                let new_str = format!("Net/ThemeName \"{}\"", LIGHT_THEME);
                theme = "light".to_string();

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(format!(
                        "{}/.xsettingsd",
                        dirs::home_dir()
                            .unwrap()
                            .into_os_string()
                            .into_string()
                            .unwrap()
                    ))
                    .unwrap();
                file.write_all(new_str.as_bytes()).unwrap();
                file.flush().unwrap();
            }
            "light" => {
                app.set_icon_from_file("./assets/moon.svg")?;

                let new_str = format!("Net/ThemeName \"{}\"", DARK_THEME);
                theme = "dark".to_string();

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(format!(
                        "{}/.xsettingsd",
                        dirs::home_dir()
                            .unwrap()
                            .into_os_string()
                            .into_string()
                            .unwrap()
                    ))
                    .unwrap();
                file.write_all(new_str.as_bytes()).unwrap();
                file.flush().unwrap();
            }
            _ => {
                println!("ok")
            }
        }

        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}
