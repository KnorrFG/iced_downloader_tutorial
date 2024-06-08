use iced::{
    alignment::Horizontal,
    widget::{button, container, row, text, text_input},
    Length, Padding, Sandbox, Settings,
};

struct Downloader {
    link_edit_contents: String,
}

#[derive(Debug, Clone)]
enum Message {
    NewDLButtonPressed,
    LinkEditChanged(String),
}

impl Sandbox for Downloader {
    type Message = Message;

    fn new() -> Self {
        Downloader {
            link_edit_contents: "".into(),
        }
    }

    fn title(&self) -> String {
        "Iced Downloader Tutorial".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NewDLButtonPressed => todo!(),
            Message::LinkEditChanged(s) => self.link_edit_contents = s,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(
            row!(
                text_input("link", &self.link_edit_contents).on_input(Message::LinkEditChanged),
                button(text("+").horizontal_alignment(Horizontal::Center))
                    .width(Length::Fixed(40.0))
                    .on_press(Message::NewDLButtonPressed)
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
