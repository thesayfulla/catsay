use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value="Meow!")]
    message: String,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    catfile: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .expect(&format!("could not read file {:?}", path));
            let cat_picture = cat_template.replace("{message}", &message.bright_red().underline());
            println!("{}", &cat_picture);
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
