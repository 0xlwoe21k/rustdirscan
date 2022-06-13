pub struct Scanner {
    url: String,
    payloads: Vec<String>,
    rand_user_agent: bool,
}

async fn get_status(url: &String) -> Result<u16, reqwest::Error> {
    let res = reqwest::get(url).await?.status().as_u16();
    Ok(res)
}

impl Scanner {
    pub fn new(_url: String, _payloads: Vec<String>, _rand_user_agent: bool) -> Scanner {
        Scanner {
            url: _url,
            payloads: _payloads,
            rand_user_agent: _rand_user_agent,
        }
    }

    pub async fn run(self) {
        println!("[*] url:{}", self.url);
        println!("[*] payload count:{}", self.payloads.len());

        for u in self.payloads {
            let furl = self.url.clone() + &u;
            let mut status_code: u16 = 0;
            if let Ok(code) = get_status(&furl).await {
                status_code = code;
            }
            if status_code == 200 {
                println!("{} --> {}", &furl, status_code);
            }
        }
    }
}
