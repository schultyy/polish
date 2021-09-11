use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "website polish")]
struct Args {
    website_url: String
}

async fn fetch_http(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let html = reqwest::get(url)
                .await?
                .text()
                .await?;
    Ok(html)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Hello {}!", args.website_url);
    fetch_http(&args.website_url)
        .await
        .and_then(|content| {
            println!("{}", content);
            Ok(())
        })
        .map_err(|err| eprintln!("{}", err))
        .ok();
    Ok(())
}