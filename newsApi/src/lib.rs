use std::error::Error;
use thiserror::Error;
use serde::Deserialize;

#[derive(thiserror::Error,Debug)]
pub enum NewsApiError{
     #[error("Failed fectching articles")]
     RequestFailed(ureq::Error),
     #[error("Failed to convert response to String")]
     FailedToresponseToString(std::io::Error),
     #[error("Article Parsing Failed")]
     FailedParsingArticle(serde_json::Error)
} 
#[derive(Deserialize,Debug)]
pub struct Articles {
    pub articles : Vec<Article>
}

#[derive(Deserialize,Debug)]
pub struct Article{
    pub title : String,
    pub url: String
}

pub fn get_articles(url : &str) -> Result<Articles,NewsApiError>{
    let response= ureq::get(url).call().map_err(|e|NewsApiError::RequestFailed(e))?.into_string().map_err(|e|NewsApiError::FailedToresponseToString(e))?;

    let articles: Articles = serde_json::from_str(&response).map_err(|e|NewsApiError::FailedParsingArticle(e))?;

    Ok(articles)
}
enum Endpoint{
    TopHeadlines
}
enum Country{
    Us
}

struct NewsApi{
    api_key: String,
    endpoint: Endpoint,
    country : Country
}
impl NewsApi {
    fn new(api_key:&str) -> NewsApi {
        NewsApi { 
            api_key: api_key.to_uppercase(), 
            endpoint: Endpoint::TopHeadlines, 
            country: Country::Us
        }
    }
    fn endpoint(&mut self,endpoint : Endpoint) -> () {
        self.endpoint = endpoint;
    }
}