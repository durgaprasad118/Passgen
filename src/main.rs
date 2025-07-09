use clap::Parser;
use rand::Rng;
#[derive(Parser, Debug)]
#[command(name = "passgen", version, about = "Generate a random password")]
struct Args {
    #[arg(short, long, default_value_t = 6)]
    length: usize,
    #[arg(short, long)]
    numbers: bool,
    #[arg(short, long)]
    symbols: bool,
    #[arg(short, long)]
    upper: bool,
    #[arg(short, long)]
    down: bool,
}

impl Args {
    fn validate(&self) {
        if !(self.numbers || self.symbols) {
            println!("Atleast one of the --numbers or --symbols should be included in Password");
            std::process::exit(1);
        }
    }
}
fn main() {
    let args = Args::parse();
    args.validate();
    let digits = String::from("0123456789"); //10
    let symbols = String::from("@#$%^*?|"); //10
    let lower: String = ('a'..='z').collect(); //26
    let upper: String = ('A'..='Z').collect(); //26
    let mut charset: String = String::new();
    if args.numbers {
        charset.push_str(&digits);
    }
    if args.symbols {
        charset.push_str(&symbols);
    }
    if args.upper {
        charset.push_str(&upper);
    }
    if args.down {
        charset.push_str(&lower);
    }
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = charset.chars().collect();
    let random: String = (0..args.length)
        .map(|_| chars[rng.gen_range(0..charset.len())])
        .collect();
    println!("Your password is : {}", random)
}
