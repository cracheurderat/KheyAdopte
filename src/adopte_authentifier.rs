use reqwest;
use reqwest::Client;
use reqwest::header::SetCookie;
use hyper::header::Cookie;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::io::Read;
use adopte_client::AdopteClient;

type ReqwestResult = Result<reqwest::Response, reqwest::Error>;
type Email = String;
type Password = String;

pub fn authentification(client: Client) -> Option<AdopteClient>{
    let user_informations = get_user_informations();
    let mut request_result = connect_user(user_informations, &client);
    match request_result {
        Ok(response) => Some(create_adopte_client(client, response)),
        _ => None
    }
}

fn get_user_informations() -> (String, String){
    (get_email(), get_password())
}

fn get_email() -> String {
    print!("Email : ");
    stdout().flush();
    get_input()
}

fn get_password() -> String {
    print!("Password : ");
    stdout().flush();
    get_input()
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input.pop();
    input
}

fn connect_user(user_informations: (Email, Password), client: &Client) -> ReqwestResult {
    let (email, password) = user_informations;
    let params = [("username", email), ("password", password)];    
    client.post("https://www.adopteunmec.com//auth/login")
          .form(&params)
          .send()
}

fn create_adopte_client(client: Client, response: reqwest::Response) -> AdopteClient {
    let cookies: &SetCookie = response.headers()
                                      .get::<SetCookie>()
                                      .expect("Could not find cookies");
    let cookie = get_key_value_cookies(cookies);
    AdopteClient::new(client, cookie)
}

fn get_key_value_cookies(cookies: &SetCookie) -> Cookie {
    let key_value_cookies: Vec<String> = cookies.as_slice().iter()
                                                .map(get_cookie_key_value)
                                                .collect();
    Cookie(vec![
        String::from(key_value_cookies.join("; "))
    ])

}

fn get_cookie_key_value(cookie: &String) -> String{
    cookie.splitn(2, ";")
          .collect::<Vec<&str>>()
          .first()
          .unwrap()
          .to_string()
}

