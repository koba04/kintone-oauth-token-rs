use actix_web::{web, App, HttpRequest, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use reqwest;
use serde::Deserialize;
use std::env;
use webbrowser;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/callback", web::get().to(callback))
    })
    .bind_openssl("localhost:3000", create_ssl_builder())?
    .run();

    open_page();
    server.await
}

async fn index(_req: HttpRequest) -> String {
    format!("Hello, please access /callback")
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
}

async fn callback(params: web::Query<CallbackParams>) -> String {
    let access_token = send_request(&params.code).await.unwrap();
    format!("Access token: {}", access_token)
}

fn create_ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .expect("Cannot find private key file on the current directory.");
    builder
        .set_certificate_chain_file("cert.pem")
        .expect("Cannot find private cert file on the current directory.");
    builder
}

#[derive(Deserialize, Debug)]
struct CallbackResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

async fn send_request(
    code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = get_kintone_oauth_config();
    let client = reqwest::Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", "https://localhost:3000/callback"),
        ("code", code),
        ("client_id", &config.client_id),
        ("client_secret", &config.client_secret_id),
    ];
    // println!("{:?}", d);
    let resp = client
        .post(&format!("{}/oauth2/token", config.base_url))
        .form(&params)
        .send()
        .await?;
    // println!("Response {:?}", resp);
    let json = resp.json::<CallbackResponse>().await?;
    println!("access_token: {}", json.access_token);
    Ok(json.access_token)
}

fn open_page() {
    let config = get_kintone_oauth_config();
    let url = format!(
        "{}/oauth2/authorization?client_id={}&redirect_uri=https%3A%2F%2Flocalhost%3A3000%2Fcallback&state=state1&response_type=code&scope={}",
        config.base_url,
        config.client_id,
        config.scope
    );
    webbrowser::open(&url).expect(&format!("Cannot open the URL: {}", &url));
}

struct KintoneOAuthConfig {
    base_url: String,
    client_id: String,
    client_secret_id: String,
    scope: String,
}

fn get_kintone_oauth_config() -> KintoneOAuthConfig {
    KintoneOAuthConfig {
        base_url: get_env_var("KINTONE_BASE_URL"),
        client_id: get_env_var("KINTONE_OAUTH_CLIENT_ID"),
        client_secret_id: get_env_var("KINTONE_OAUTH_CLIENT_SECRET_ID"),
        scope: get_env_var("KINTONE_OAUTH_SCOPE"),
    }
}

fn get_env_var(name: &str) -> String {
    env::var(name).expect(&format!("you should set {}", name))
}
