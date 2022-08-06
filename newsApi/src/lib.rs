#[cfg(feature = "async")]
use reqwest::Method;
use serde::Deserialize;
use url::Url;

const BASE_URL : &str =  "https://newsapi.org/v2";

#[derive(thiserror::Error,Debug)]
pub enum NewsApiError{
     #[error("Failed fectching articles")]
     RequestFailed(#[from] ureq::Error),
     #[error("Failed to convert response to String")]
     FailedToresponseToString(#[from] std::io::Error),
     #[error("Article Parsing Failed")]
     FailedParsingArticle(#[from] serde_json::Error),
     #[error("Url parsing failed")]
     UrlParsing(#[from] url::ParseError),
     #[error("Request failed: {0}")]
     BadRequest(&'static str),
     #[error("Async request failed")]
     #[cfg(feature = "async")]
     AsyncRequestFailed(#[from] reqwest::Error)
} 
#[derive(Deserialize,Debug)]
pub struct NewsApiResponse {
    status:String,
    pub articles : Vec<Article>,
    code:Option<String>
}
impl NewsApiResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Deserialize,Debug)]
pub struct Article{
    title : String,
    url: String
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}

/* pub fn get_articles(url : &str) -> Result<Articles,NewsApiError>{
    let response= ureq::get(url).call().map_err(|e|NewsApiError::RequestFailed(e))?.into_string().map_err(|e|NewsApiError::FailedToresponseToString(e))?;

    let articles: Articles = serde_json::from_str(&response).map_err(|e|NewsApiError::FailedParsingArticle(e))?;

    Ok(articles)
} */
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

pub struct NewsApi{
    api_key: String,
    endpoint: Endpoint,
    country : Country
}
impl NewsApi {
    pub fn new(api_key:&str) -> NewsApi {
        NewsApi { 
            api_key: api_key.to_uppercase(), 
            endpoint: Endpoint::TopHeadlines, 
            country: Country::Us
        }
    }

    pub fn endpoint(&mut self,endpoint : Endpoint) -> &mut NewsApi {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self,country:Country) -> &mut NewsApi {
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
    pub fn fetch(&self) -> Result<NewsApiResponse,NewsApiError>{
        let url :String = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response : NewsApiResponse = req.call()?.into_json()?;
        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(NewsApi::map_response_error(response.code))
        } 
        //todo!() 
    }
    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) -> Result<NewsApiResponse,NewsApiError>{
        let url = self.prepare_url()?;
        /* dbg!(&url);  */
        let client = reqwest::Client::new();
        let request = client
        .request(Method::GET,url)
        .header("Authorization", &self.api_key)
        .build()
        .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;
        
        let testRequest = client.get("https://newsapi.org/v2/everything?q=keyword&apiKey=16fbc4733a3d4a43b6da9a7a909bb1de").send().await?;
        dbg!(testRequest);

        let response:NewsApiResponse = client
        .execute(request)
        .await?
        .json()
        .await
        .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(NewsApi::map_response_error(response.code))
        } 
    }
    fn map_response_error(code : Option<String>) -> NewsApiError {
        if let Some(code ) = code{
            match code.as_str() {
                "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled."),
                "apiKeyExhausted" => NewsApiError::BadRequest("Your API key has no more requests available."),
                "apiKeyInvalid" => NewsApiError::BadRequest("Your API key hasn't been entered correctly. Double check it and try again."),
                "parametersMissing" => NewsApiError::BadRequest("Required parameters are missnig from the request."),
                "parameterInvalid" => NewsApiError::BadRequest("You've included a parameter in your request which is currently not supported"),
                _ => NewsApiError::BadRequest("Unknown error")
            }
        }else{
            NewsApiError::BadRequest("Unknown error")
        }
    }

}