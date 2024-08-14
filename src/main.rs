mod art;
mod quote;
mod bubble;

use clap::{arg, ArgAction, Command};

fn main() {
    let matches = Command::new("catsay")
        .version("1.0")
        .author("Sayfulla Mirxalikov")
        .about("Displays ASCII cats art or a random quote")
        .arg(
            arg!(
                -o --option <OPTION> "Sets the ASCII image option"
            )
            .required(false)
            .value_parser(art::get_options())
            .default_value("cat"),
        )
        .arg(
            arg!(
                --quote "Fetches and displays a random quote"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(arg!([message] "Message to be displayed").required(false).default_value("Hello!"))
        .get_matches();
    if matches.get_flag("quote") {
        match quote::fetch_quote() {
            Ok(quote) => {
                if let Some(option) = matches.get_one::<String>("option") {
                    let bubble = bubble::make_bubble(&quote);
                    match art::get_art(option) {
                        Some(image) => println!("{}\n{}", bubble, image),
                        None => println!("Option not found."),
                    }
                }
            },
            Err(e) => eprintln!("Failed to fetch quote: {}", e),
        }
    } else {
        if let Some(option) = matches.get_one::<String>("option") {
            let message = matches
                .get_one::<String>("message").unwrap().as_str();

            let bubble = bubble::make_bubble(message);

            match art::get_art(option) {
                Some(image) => println!("{}\n{}", bubble, image),
                None => println!("Option not found."),
            }
        }
    }
}
