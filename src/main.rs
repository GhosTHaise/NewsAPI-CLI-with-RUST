use colour::{dark_green,yellow};
use std::error::Error;
use newsApi::{get_articles,Articles};

fn render_articles(articles:&Articles)-> (){
    for i in &articles.articles {
        dark_green!("> {}\n",i.title);
        yellow!("- {}\n\n",i.url);
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    let url = "https://newsapi.org/v2/everything?q=tesla&sortBy=publishedAt&apiKey=16fbc4733a3d4a43b6da9a7a909bb1de";
    let _articles = get_articles(url)?;
    render_articles(&_articles);
    Ok(())
}
