use super::{Driver,Config};
use super::Response as SelfResponse;
use serde::{Deserialize};
use serde_json;
use reqwest;
use reqwest::header::*;

const URL : &'static str  = "https://openapi.naver.com/v1/papago/n2mt";








struct PapagoDriver {
    client_id : String,
    client_secret : String,
}
#[derive(Deserialize)]
struct PapagoConfig {
    client_id : String,
    client_secret : String,
}



impl Config for PapagoConfig {
    fn new(source : String) -> Result<Self,String> where Self : Sized {
        let res : 
            Result<PapagoConfig,String> = match serde_json::from_str(source.as_str()) {
            
            Ok(ok) => Result::Ok(ok),
            Err(err) => Result::Err(err.to_string())
        };
        res 
    }
    fn getAttr(&self,name : &str) -> String {
        match name {
            "client_id" => self.client_id.clone(),
            "client_secret" => self.client_secret.clone(),
            _ => String::from(""),
        }
    }
}

const CLIENT_ID : &str= "X-Naver-Client-Id";
const CLIENT_SECRET : &str = "X-Naver-Client-Secret";

impl PapagoDriver {
    fn headers(&self) -> HeaderMap {
        let mut header  = HeaderMap::new();



        header.insert(CONTENT_TYPE,HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
        header.insert(CLIENT_ID,self.client_id.as_str().parse().unwrap());
        header.insert(CLIENT_SECRET,self.client_secret.as_str().parse().unwrap());
        header
    }

    fn body(&self,word : &String) -> String {
        format!("source=en&target=ko&text={}",word)
    }
}

impl Driver for PapagoDriver {
    fn new(config : impl Config) -> Self {
        PapagoDriver {
            client_id : config.getAttr("client_id"),
            client_secret : config.getAttr("client_secret")
        }
    }



    fn trans(&self,word : String) -> SelfResponse {
        let client = reqwest::blocking::Client::new();
        
        let res = client.post(URL)
            .headers(self.headers())
            .body(self.body(&word))
            .send().expect("can't connect server");
        
        SelfResponse {
            statusCode : res.status().as_u16(),
            data : res.text().unwrap()
        }
        
    }
}