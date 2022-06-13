use std::path::PathBuf;
use std::time::{ Instant};
use structopt::StructOpt;
mod scanner;
mod util;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustdirscan", about = "web sensitive information scanner.")]
struct Opt {
    #[structopt(short = "v")]
    url: String,

    #[structopt(short = "p", parse(from_os_str))]
    poc_path: PathBuf,

    /// random user agent on http request
    #[structopt(short, long)]
    rand_user_agent: bool,
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let opt = Opt::from_args();
    let mut file_path = "";
    if let Some(p) = opt.poc_path.to_str() {
        file_path = p;
    }

    let mut payloads: Vec<String> = Vec::new();
    if let Some(f) = util::get_payloads(file_path) {
        payloads = f;
    }

    let scanner = scanner::Scanner::new(opt.url, payloads, opt.rand_user_agent);
    scanner.run().await;

    println!("time: {}", now.elapsed().as_secs());
}
