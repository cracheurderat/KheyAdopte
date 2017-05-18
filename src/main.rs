extern crate reqwest;
extern crate json;
#[macro_use] extern crate hyper;


mod adopte_authentifier;
mod adopte_client;
use adopte_client::AdopteClient;

use std::io::Read;
use reqwest::Client;
use reqwest::header::SetCookie;
use reqwest::RedirectPolicy;

fn main() {
    /*
    let mut content = String::new();
    let mut res = client.post("https://www.adopteunmec.com//auth/login")
                        .body("username=xxxx;password=xxxx;remember=on")
                        .send().unwrap();
    let readed = res.read_to_string(&mut content);
    let mut http_client: Client = Client::new().unwrap();
    let mut adopte_client: AdopteClient = AdopteClient::new(http_client);

    let mut content2 = String::new();
    let mut res2 = reqwest::get("https://www.adopteunmec.com/home").unwrap();
    res2.read_to_string(&mut content2);
    
    //print!("{:?}", res2);
    print!("{:?}", res2.headers().get::<SetCookie>());

    adopte_client
    */

    let mut http_client: Client = Client::new().unwrap();
    http_client.redirect(RedirectPolicy::none());
    let mut adopte_client: AdopteClient = adopte_authentifier::authentification(http_client).unwrap();
    adopte_client.visit_girls();
    

}
