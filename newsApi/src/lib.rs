use std::error::Error;
use thiserror::Error;
use serde::Deserialize;
use url::Url;

const BASE_URL : &str =  "https://newsapi.org/v2";

#[derive(thiserror::Error,Debug)]
pub enum NewsApiError{
     #[error("Failed fectching articles")]
     RequestFailed(ureq::Error),
     #[error("Failed to convert response to String")]
     FailedToresponseToString(std::io::Error),
     #[error("Article Parsing Failed")]
     FailedParsingArticle(serde_json::Error),
     #[error("Url parsing failed")]
     UrlParsing(#[from] url::ParseError)
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
pub enum Endpoint{
    TopHeadlines
}

impl ToString for Endpoint{
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

pub enum Country{
    Us
}
impl ToString for Country{
    fn to_string(&self) -> String {
        match  self {
            Self::Us => "us".to_string()
        }
    }
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

    fn endpoint(&mut self,endpoint : Endpoint) -> &mut NewsApi {
        self.endpoint = endpoint;
        self
    }

    fn country(&mut self,country:Country) -> &mut NewsApi {
        self.country = country;
        self
    }

    fn prepare_url(&self) -> Result<String,NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());
        let country = format!("country={}",&self.country.to_string());
        url.set_query(Some(&country));
        Ok(url.to_string())
    }

}