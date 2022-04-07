use std::fs;

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
    let theme = file_content.split("\"").collect::<Vec<&str>>()[1];

    let icon_file = if theme.to_lowercase().contains("dark") {
        "./assets/moon.svg"
    } else {
        "./assets/sun.svg"
    };

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_file(icon_file)?;

    app.add_menu_item("Change theme", |_| {
        println!("Changing theme");
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
