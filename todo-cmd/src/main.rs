mod cmd;
use cmd::Cmd;
use dialoguer::{theme::ColorfulTheme, Input};

fn main() {
    println!("Welcome to todo. Your command line todo pal!");

    let mut cmd = Cmd::new();

    cmd.help();

    loop {
        let input = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(">>>")
            .interact_text()
            .unwrap();

        cmd.parse_commands(input);
    }
}
