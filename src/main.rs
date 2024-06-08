use iced::{
    alignment::Horizontal,
    widget::{
        button, column, container, horizontal_rule, row, scrollable, text, text_input, Column,
        Container, Text,
    },
    Alignment, Element, Length, Padding, Renderer, Sandbox, Settings, Theme,
};

struct Downloader {
    link_edit_contents: String,
    downloads: Vec<Download>,
}

#[derive(Debug, Clone)]
enum Message {
    NewDLButtonPressed,
    LinkEditChanged(String),
}

struct Download {
    url: String,
    _state: DownloadState,
}

enum DownloadState {
    Starting,
}

impl Sandbox for Downloader {
    type Message = Message;

    fn new() -> Self {
        Downloader {
            link_edit_contents: "".into(),
            downloads: vec![],
        }
    }

    fn title(&self) -> String {
        "Iced Downloader Tutorial".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NewDLButtonPressed => {
                self.downloads.push(Download {
                    url: self.link_edit_contents.clone(),
                    _state: DownloadState::Starting,
                });
                self.link_edit_contents.clear();
            }
            Message::LinkEditChanged(s) => self.link_edit_contents = s,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        main_container(column!(self.new_download_row(), self.downloads(),).spacing(20)).into()
    }
}

impl Downloader {
    fn new_download_row(&self) -> iced::widget::Row<'_, Message> {
        row!(
            text("New Download: "),
            text_input("link", &self.link_edit_contents).on_input(Message::LinkEditChanged),
            button(text("+").horizontal_alignment(Horizontal::Center))
                .width(Length::Fixed(40.0))
                .on_press(Message::NewDLButtonPressed)
        )
        .align_items(Alignment::Center)
        .spacing(20)
    }

    fn downloads(&self) -> Column<Message> {
        let sections = self
            .downloads
            .iter()
            .map(|dl| column!(text(&dl.url), text("Starting ...")));

        let mut section_with_bars: Vec<Element<Message>> = sections
            .flat_map(|elem| [elem.into(), horizontal_rule(2).into()])
            .collect();
        section_with_bars.pop();

        column!(
            text("Downloads").size(32),
            container(scrollable(
                container(column(section_with_bars).spacing(10))
                    .padding(Padding::from([0, 30]))
                    .width(Length::Fill)
            ))
            .padding(Padding::from([10, 30]))
        )
        .into()
    }
}

fn main_container<'a, T>(content: T) -> Container<'a, Message>
where
    T: Into<Element<'a, Message, Theme, Renderer>>,
{
    container(content)
        .padding(Padding::from([30, 80]))
        .center_x()
        .center_y()
}

fn main() {
    Downloader::run(Settings::default()).unwrap()
}
