pub struct Scanner {
    url: String,
    payload: Vec<String>,
}

impl Scanner {
    pub fn new(_url: String, _payloads: Vec<String>) -> Scanner {
        Scanner {
            url: _url,
            payload: _payloads,
        }
    }
}
