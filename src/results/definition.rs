#[derive(Debug)] // this allows to print MenuChoice data
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "main" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

/*fn print_choice(choice: &Result<MenuChoice, String>) {
    println!("choice = {:?}", choice);
}*/

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

pub fn master(show: bool) {
    if show {
        println!("\n--- Results Definition");

        let choice = get_choice("main");
        //print_choice(&choice);
        match choice {
            Ok(inner_choice) => print_choice(&inner_choice),
            Err(e) => println!("error = {:?}", e),
        }
    }
}