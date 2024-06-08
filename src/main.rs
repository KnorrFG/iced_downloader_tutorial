use iced::{widget::text, Sandbox, Settings};

struct Downloader;

#[derive(Debug)]
enum Message {}

impl Sandbox for Downloader {
    type Message = Message;

    fn new() -> Self {
        Downloader
    }

    fn title(&self) -> String {
        "Iced Downloader Tutorial".into()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        text("Hello World").into()
    }
}

fn main() {
    Downloader::run(Settings::default()).unwrap()
}
