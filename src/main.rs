use iced::{
    alignment::Horizontal,
    widget::{button, container, row, text, text_input},
    Length, Padding, Sandbox, Settings,
};

struct Downloader;

#[derive(Debug, Clone)]
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
        container(
            row!(
                text_input("link", ""),
                button(text("+").horizontal_alignment(Horizontal::Center))
                    .width(Length::Fixed(40.0))
            )
            .spacing(20),
        )
        .padding(Padding::from([30, 80]))
        .center_x()
        .center_y()
        .into()
    }
}

fn main() {
    Downloader::run(Settings::default()).unwrap()
}
