pub mod theme;

use std::{error::Error};
use newsApi::{get_articles,Articles};
use dotenv::dotenv;

fn render_articles(articles:&Articles)-> (){
    let theme = theme::default();
    theme.print_text("# top headlines\n\n"); 
    for i in &articles.articles {
        theme.print_text(&format!("`{}`",i.title));
        theme.print_text(&format!("> *{}*",i.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let url = "https://newsapi.org/v2/everything?q=tesla&sortBy=publishedAt&apiKey=";
    let url = format!("{}{}",url,api_key);
    let _articles = get_articles(&url)?;
    render_articles(&_articles);
    Ok(())
}


