use colour::{dark_green,yellow};
use std::{error::Error};
use newsApi::{get_articles,Articles};
use dotenv::dotenv;

fn render_articles(articles:&Articles)-> (){
    for i in &articles.articles {
        dark_green!("> {}\n",i.title);
        yellow!("- {}\n\n",i.url);
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
