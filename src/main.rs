mod cats;
use cats::CatChoice;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    #[structopt(default_value = "Meow!")]
    pub message: String,

    #[structopt(short = "c", long = "cat", parse(from_os_str))]
    pub cat: Option<CatChoice>,

    #[structopt(short = "l", long = "list")]
    pub list: bool,
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

    let message = options.message;

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    match options.cat {
        Some(cat_choice) => {
            let cat_picture = match cat_choice {
                CatChoice::Felix => {
                    format!(
                        "{}
 \\
  \\
     /\\_/\\
    ( O O )
    =( I )=",
                        message
                    )
                }
                CatChoice::Whiskers => {
                    format!(
                        "{}
 \\
  \\
     /\\_/\\
    ( o.o )
    > ^ <",
                        message
                    )
                }
                CatChoice::Mittens => {
                    format!(
                        "{}
 \\
  \\
     /\\_/\\
    ( -.- )
    O(\"(\")(\")",
                        message
                    )
                }
            };
            println!("{}", cat_picture);
        }
        None => {
            println!("{}", message);
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( O O )");
            println!("    =( I )=");
        }
    }
}
