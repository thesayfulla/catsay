use std::ffi::OsStr;
use structopt::StructOpt;

#[derive(Debug)]
pub enum CatChoice {
    Felix,
    Whiskers,
    Mittens,
}

impl From<&OsStr> for CatChoice {
    fn from(os_str: &OsStr) -> Self {
        match os_str.to_str() {
            Some("felix") => CatChoice::Felix,
            Some("whiskers") => CatChoice::Whiskers,
            Some("mittens") => CatChoice::Mittens,
            _ => panic!("Invalid cat choice"),
        }
    }
}

impl CatChoice {
    fn draw(&self, message: &str) -> String {
        match self {
            CatChoice::Felix => format!(
                "{}\n \\ \n     /\\_/\\\n    ( o.o )\n    > ^ <",
                message
            ),
            CatChoice::Whiskers => format!(
                "{}\n \\ \n     /\\_/\\\n    ( -.- )\n    O(\"(\")(\")",
                message
            ),
            CatChoice::Mittens => format!(
                "{}\n \\ \n     /\\_/\\\n    ( O O )\n    =( I )=",
                message
            ),
        }
    }
}

#[derive(StructOpt)]
pub struct Options {
    #[structopt(default_value = "Meow!")]
    pub message: String,

    #[structopt(short = "c", long = "cat", parse(from_os_str))]
    pub cat: Option<CatChoice>,

    #[structopt(short = "l", long = "list")]
    pub list: bool,
}

fn print_cat_picture(cat_picture: &str) {
    println!("{}", cat_picture);
}

fn print_default_cat(message: &str) {
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( O O )");
    println!("    =( I )=");
}

fn main() {
    let options = Options::from_args();

    if options.list {
        println!("Available cat choices:");
        println!("felix");
        println!("whiskers");
        println!("mittens");
        return;
    }

    let message = options.message.to_lowercase();

    if message == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    match options.cat {
        Some(cat_choice) => {
            print_cat_picture(&cat_choice.draw(&options.message));
        }
        None => print_default_cat(&options.message),
    }
}
