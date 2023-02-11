mod theme;

use std::error::Error;
use dotenv::dotenv;
use news_api::{Article, Country, Endpoint, NewsAPI};

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();

    theme.print_text("# Top headlines\n\n");

    for i in articles {
        theme.print_text(&format!("`{}`", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = std::env::var("API_KEY")?;

    let mut news_api = NewsAPI::new(&api_key);

    news_api
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::Us);

    let news_api_response = news_api.fetch_async().await?;

    render_articles(&news_api_response.articles());

    Ok(())
}
