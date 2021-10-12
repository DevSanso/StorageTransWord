use super::{Driver,Config};
use super::Response as SelfResponse;
use serde::{Deserialize};
use serde_json;
use serde_json::{Value};
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

    fn body(word : &String) -> String {
        format!("source=en&target=ko&text={}",word)
    }
    fn extract_trans_text_in_res_body(raw : String) -> String {
        let val : Value =  serde_json::from_str(raw.as_str()).unwrap();
        
        val["message"]["result"]["translatedText"].to_string()
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
            .body(PapagoDriver::body(&word))
            .send().expect("can't connect server");
        
        SelfResponse {
            status_code : res.status().as_u16(),
            data : PapagoDriver::extract_trans_text_in_res_body(res.text().unwrap())
        }
        
    }
}

#[cfg(test)]
mod tests {
    use std::include_str;
    use super::*;
    #[test]
    fn run_papago() {
        let source = include_str!("../../test_config/papago.json");
        let driver : PapagoDriver;
        {
            let config = PapagoConfig::new(String::from(source)).unwrap();
            driver = PapagoDriver::new(config);
        }
        let res = driver.trans(String::from("hello"));
        println!("{} : {}",res.status_code,res.data);
    }
}