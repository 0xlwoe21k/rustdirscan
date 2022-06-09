use std::path::PathBuf;

use structopt::StructOpt;

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
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
