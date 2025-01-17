use std::error::Error;

use clipstash::{
    domain::clip::field::{Content, Expires, Password, Title},
    service::ask::{GetClip, NewClip, UpdateClip},
    web::api::{ApiKey, API_KEY_HEADER},
    Clip, ShortCode,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    Get {
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<String>,
    },
    New {
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
    Update {
        shortcode: ShortCode,
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "clipclient", about = "ClipStash API Client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8000", env = "CLIPSTASH_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key: ApiKey,
}

fn get_clip(addr: &str, ask_svc: GetClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip/{}", addr, ask_svc.shortcode.into_inner());
    let request = client.get(addr);
    let request = match ask_svc.password.into_inner() {
        Some(password) => request.header(reqwest::header::COOKIE, format!("password={}", password)),
        None => request,
    };
    let request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.send()?.json()?)
}

fn new_clip(addr: &str, ask_svc: NewClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let request = client.post(addr);
    let request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn update_clip(addr: &str, ask_svc: UpdateClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let request = client.put(addr);
    let request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get {
            shortcode,
            password,
        } => {
            let req = GetClip {
                password: Password::new(password.unwrap_or_default())?,
                shortcode,
            };
            let clip = get_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
        Command::New {
            clip,
            password,
            expires,
            title,
        } => {
            let req = NewClip {
                content: Content::new(clip.as_str())?,
                title: title.unwrap_or_default(),
                password: password.unwrap_or_default(),
                expires: expires.unwrap_or_default(),
            };
            let clip = new_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
        Command::Update {
            shortcode,
            clip,
            password,
            expires,
            title,
        } => {
            let password = password.unwrap_or_default();
            let req = GetClip {
                password: password.clone(),
                shortcode: shortcode.clone(),
            };

            let original_clip = get_clip(opt.addr.as_str(), req, opt.api_key.clone())?;

            let req = UpdateClip {
                shortcode,
                password,
                content: Content::new(clip.as_str())?,
                expires: expires.unwrap_or(original_clip.expires),
                title: title.unwrap_or(original_clip.title),
            };
            let clip = update_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e);
    }
}
