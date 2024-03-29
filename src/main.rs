use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(long)]
    from: String,

    #[arg(long)]
    to: String,

    #[arg(long)]
    file: String,
}

fn main() {
     let args = Args::parse();

    println!("{:?}", args);

}
