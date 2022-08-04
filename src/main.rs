pub mod theme;

use std::{error::Error};
use dotenv::dotenv;
use newsApi::{NewsApi, Article,Endpoint,Country};

fn render_articles(articles:&Vec<Article>)-> (){
    let theme = theme::default();
    theme.print_text("# top headlines\n\n"); 
    for i in articles {
        theme.print_text(&format!("`{}`",i.title()));
        theme.print_text(&format!("> *{}*",i.url()));
        theme.print_text("---");
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let mut newsApi = NewsApi::new(&api_key);
    newsApi.endpoint(Endpoint::TopHeadlines).country(Country::Us);
    let newsApi_reponse  = newsApi.fetch()?;
    /* let url = "https://newsapi.org/v2/everything?q=tesla&sortBy=publishedAt&apiKey=";
    let url = format!("{}{}",url,api_key);
    let _articles = get_articles(&url)?; */
    render_articles(&newsApi_reponse.articles());
    Ok(())
}


