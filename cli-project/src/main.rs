use clap::Parser;
use colored::Colorize;

#[derive(Parser)] // attribute
// struct for defining our command line arguments definition
struct Options {
    #[clap(default_value = "Meow!")]
    /// what does the cat say?
    message: String, // [1]

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let options = Options::parse(); // [2]
    let message =options.message;

    let eye = if options.dead {
        "x"
    } else {
        "o"
    };


    //STDERR output example
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    // println!("{}",
    // message.bright_yellow().underline().on_purple()
    // );
    //
    // println!(" \\");
    // println!("  \\");
    // println!("      /\\_/\\");
    // println!("      ( {eye}  {eye} )", eye=eye.red().bold());
    // println!("      =( I )=");

    match &options.catfile {
        Some(path) => {
             let cat_template = std::fs::read_to_string(path)?;

            // let cat_template = match std::fs::read_to_string(path){
            //     Ok(file_content) => file_content,
            //     Err(e) => return e.into(), // e:std::io::Error
            // };

            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);

            println!("{}",
            message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);

        }

        None => {
            // ... print the cat as before

            println!(" \\");
            println!("  \\");
            println!("      /\\_/\\");
            println!("      ( {eye}  {eye} )", eye=eye.red().bold());
            println!("      =( I )=");
        }
    }

    Ok(())

}


// Using std::env::args to get the command line argument
// fn using_std_env() {
//     let message = std::env::args().nth(1).expect("Missing the message. Usage: catsay <message>");
//
//     println!("{}", message);
//     println!(" \\");
//     println!("  \\");
//     println!("      /\\_/\\");
//     println!("      ( O  O )");
//     println!("      =( I )=");
//
// }