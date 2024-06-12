use clap::{Arg, ArgAction, Command};

// "Builder" pattern
fn main() {
    // Can (should) this be tucked away in a fn?
    let matches = Command::new("echor") 
        .version("0.1.0") 
        .author("Ken Youens-Clark <kyclark@gmail.com>") 
        .about("Rust version of `echo`") 
        .arg(Arg::new("text")
                  .value_name("TEXT")
                  .help("Input text")
                  .required(true)
                  .num_args(1..),
        )
        .arg(Arg::new("omit_newline")
                  .short('n')
                  .action(ArgAction::SetTrue)
                  .help("Do not emit newline"),
                  )
        .get_matches(); 
    let text:Vec<String>  = 
        //TODO: How do I know unwrap() will not panic?
        matches.get_many("text").unwrap().cloned().collect();
    let omit_newline  = matches.get_flag("omit_newline");

    // C looks like this:
    // ending = omit_newline ? "" : "\n";
    // rust looks like this:
    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{ending}", text.join(" "));

}

