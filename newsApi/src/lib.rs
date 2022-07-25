
use serde::Deserialize;

#[derive(thiserror::Error)]
enum NewsApiError{
     #[error("Failed fectching articles")]
     RequestFailed,
     #[error("Failed to convert response to String")]
     FailedToresponseToString,
     #[error("Article Parsing Failed")]
     FailedParsingArticle
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
    let response= ureq::get(url).call().map_err(|e|NewsApiError::RequestFailed)?.into_string().map_err(|e|NewsApiError::FailedToresponseToString)?;

    let articles: Articles = serde_json::from_str(&response).map_err(|e|NewsApiError::FailedParsingArticle)?;

    Ok(articles)
}