pub mod theme;

use std::{error::Error};
use dotenv::dotenv;
use newsApi::{NewsApi, NewsApiResponse};
use newsApi::{Endpoint,Country};

fn render_articles(articles:&Vec<Articles>)-> (){
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
    let newsApi = NewsApi::new(&api_key);
    newsApi.endpoint(Endpoint::TopHeadlines).country(Country::Us);
    let articles : NewsApiResponse = newsApi.fetch();
    /* let url = "https://newsapi.org/v2/everything?q=tesla&sortBy=publishedAt&apiKey=";
    let url = format!("{}{}",url,api_key);
    let _articles = get_articles(&url)?; */
    render_articles(&articles);
    Ok(())
}


