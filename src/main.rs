#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use clap::Clap;

mod html_document;
mod validator;

#[derive(Clap, Debug)]
#[clap(name = "website polish")]
struct Args {
    website_url: String,
    keywords: Vec<String>
}

async fn fetch_http(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let html = reqwest::get(url)
                .await?
                .text()
                .await?;
    Ok(html)
}

fn validate_website(html: &str, keywords: Vec<String>) {
    let title = html_document::HtmlDocument::new(html)
                                .website_title()
                                .unwrap_or_default();
    let validator = validator::Validator::new(keywords);
    println!("Validating Website");

    let results = validator.validate(&title);
    if results.len() == 0 {
        println!("No validation errors. Quitting early.");
        return;
    }
    let mut table = Table::new();
    table.add_row(row!["Number", "Error"]);
    for (i, result) in results.iter().enumerate() {
        table.add_row(row![i, result.message]);
    }
    println!("Results:\n");
    table.printstd();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Analyzing {}!", args.website_url);
    fetch_http(&args.website_url)
        .await
        .and_then(|content| {
            Ok(validate_website(&content, args.keywords))
        })
        .map_err(|err| eprintln!("{}", err))
        .ok();
    Ok(())
}