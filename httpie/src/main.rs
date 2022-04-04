use anyhow::{anyhow, Ok, Result};
use clap::{Args, Command, Parser, Subcommand};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Request, Response, Url};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::str::FromStr;

#[derive(Parser)]
#[clap(author, version, about,long_about = None)]
#[clap(propagate_version = true)]
struct App {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get(Get),
    Post(Post),
}

#[derive(Args, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(Args, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).into(),
            v: (split.next().ok_or_else(err)?).into(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::parse();
    let client = Client::new();
    let result = match app.command {
        Commands::Get(get) => get.request(client).await?,
        Commands::Post(post) => post.request(client).await?,
    };

    Ok(result)
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    return Ok(_url.into());
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

impl Get {
    async fn request(&self, client: Client) -> Result<()> {
        let resp = client.get(&self.url).send().await?;
        print_resp(resp).await?;
        Ok(())
    }
}

impl Post {
    async fn request(&self, client: Client) -> Result<()> {
        let mut body = HashMap::new();
        for pair in self.body.iter() {
            body.insert(&pair.k, &pair.v);
        }
        let resp = client.post(&self.url).json(&body).send().await?;
        print_resp(resp).await?;
        Ok(())
    }
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status)
}

fn print_headers(resp: &Response) {
    for (k, v) in resp.headers() {
        println!("{} {:?}", k.to_string().green(), v)
    }
}

// todo highlight json by syntect
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => {
            println!("{}", body)
        }
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).map(|v| {
        let mut split = v.to_str().unwrap().split(";");
        if let Some(it) = split.next() {
            return it.parse().unwrap();
        }
        return mime::TEXT_PLAIN;
    })
}

// 条件编译：整个mod只有在 cargo test 时才编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}
