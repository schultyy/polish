use clap::Clap;

mod html_document;

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
    println!("Analyzing {}!", args.website_url);
    fetch_http(&args.website_url)
        .await
        .and_then(|content| {
            let title = html_document::HtmlDocument::new(&content)
                                .website_title()
                                .unwrap_or_default();
            println!("WEBSITE TITLE: {}", title);
            Ok(())
        })
        .map_err(|err| eprintln!("{}", err))
        .ok();
    Ok(())
}