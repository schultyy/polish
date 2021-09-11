use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "website polish")]
struct Args {
    website_url: String
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.website_url);
}