use reqwest;
use reqwest::Client;
use std::io::Read;
use json;
use json::{Result, Array};
use json::JsonValue;
use hyper::header::{Headers, Cookie};

header! { (XRequestedWith, "X-Requested-With") => [String] }

const BASE_URL: &str = "https://www.adopteunmec.com";

#[derive(Debug)]
pub struct GirlProfileCounter{
    offset: usize, 
    count: usize,
}

impl GirlProfileCounter {
    pub fn new(offset: usize, count: usize) -> Self {
        GirlProfileCounter{
            offset,
            count
        }
    }

    pub fn update(&mut self){
        self.offset += self.count;
    }

    pub fn set_offset(&mut self, offset: usize){
        self.offset = offset;
    }

    pub fn set_count(&mut self, count: usize){
        self.count = count;
    }
}

#[derive(Debug)]
pub struct AdopteClient{
    client: Client, 
    cookie: Cookie,
    counter: GirlProfileCounter
}

impl AdopteClient {
     pub fn new(client: Client, cookie: Cookie) -> Self {
         let counter = GirlProfileCounter::new(0, 12);
         AdopteClient{
             client,
             cookie,
             counter
         }
     }

     pub fn visit_girls(&mut self){
        loop{
            if let Ok(JsonValue::Array(profiles)) = self.get_girls_profiles(){
                for profile in profiles.iter() {
                    if let JsonValue::Short(url) = profile["url"] {
                        self.visit_girl(url.as_str());
                    }
                }
            }
        }
     }

     fn visit_girl(&self, profil_url: &str){
         let response = self.url_get(format!("{}{}",BASE_URL, profil_url));
         println!("{}", response.status());
     }

     fn get_girls_profiles(&mut self) -> Result<json::JsonValue> {
        let mySearch = "/mySearch/more?".to_owned();
        let params = format!("offset={}&count={}", self.counter.offset, self.counter.count);
        let mut response = self.url_get_ajax(format!("{}{}{}",BASE_URL, mySearch, params ));
        self.counter.update();
        let mut json_response = String::new();
        response.read_to_string(&mut json_response);
        json::parse(&json_response)
     }

     fn url_get_ajax(&self, url: String) -> reqwest::Response {
        self.client
            .get(&(url))
            .header(self.cookie.clone())
            .header(XRequestedWith("XMLHttpRequest".to_owned()))
            .send().unwrap()
     }

    fn url_get(&self, url: String) -> reqwest::Response {
        self.client
            .get(&(url))
            .header(self.cookie.clone())
            .send().unwrap()
     }
}

