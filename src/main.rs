use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use iced::{
    alignment::Horizontal,
    executor,
    futures::{Stream, StreamExt},
    widget::{
        button, column, container, horizontal_rule, image, progress_bar, row, scrollable, text,
        text_input, Column, Container,
    },
    Alignment, Application, Command, Element, Length, Padding, Renderer, Settings, Theme,
};

use async_stream::try_stream;
use humansize::{format_size, BINARY};

struct Downloader {
    link_edit_contents: String,
    downloads: Vec<Download>,
}

#[derive(Debug, Clone)]
enum Message {
    NewDLButtonPressed,
    LinkEditChanged(String),
    DownloadStarted { id: usize, opt_size: Option<u64> },
    DownloadProgress { id: usize, bytes: usize },
    DownloadFinished { id: usize },
    DownloadErrored { id: usize, message: String },
}

struct Download {
    url: String,
    state: DownloadState,
}

enum DownloadState {
    Starting,
    Downloading {
        downloaded_bytes: usize,
        complete_size: Option<u64>,
    },
    Done {
        completed_size: usize,
    },
    Error {
        message: String,
    },
}

impl Application for Downloader {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Downloader {
                link_edit_contents: "".into(),
                downloads: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Iced Downloader Tutorial".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::NewDLButtonPressed => {
                let cmd = Command::run(
                    create_download_stream(self.link_edit_contents.clone(), self.downloads.len()),
                    |x| x,
                );
                self.downloads.push(Download {
                    url: self.link_edit_contents.clone(),
                    state: DownloadState::Starting,
                });
                self.link_edit_contents.clear();
                cmd
            }
            Message::LinkEditChanged(s) => {
                self.link_edit_contents = s;
                Command::none()
            }
            Message::DownloadStarted { id, opt_size } => {
                self.downloads[id].state = DownloadState::Downloading {
                    downloaded_bytes: 0,
                    complete_size: opt_size,
                };
                Command::none()
            }
            Message::DownloadProgress { id, bytes } => {
                if let DownloadState::Downloading {
                    downloaded_bytes, ..
                } = &mut self.downloads[id].state
                {
                    *downloaded_bytes += bytes;
                } else {
                    panic!("Got a progress update for a download that is not downloading");
                }
                Command::none()
            }
            Message::DownloadFinished { id } => {
                let DownloadState::Downloading {
                    downloaded_bytes, ..
                } = self.downloads[id].state
                else {
                    panic!("Got done message for a download that is not downloading");
                };
                self.downloads[id].state = DownloadState::Done {
                    completed_size: downloaded_bytes,
                };
                Command::none()
            }
            Message::DownloadErrored { id, message } => {
                self.downloads[id].state = DownloadState::Error { message };
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        main_container(column!(self.new_download_row(), self.downloads(),).spacing(20)).into()
    }
}

impl Downloader {
    /// creates the upmost part of the gui that allow for adding new downloads
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

    /// creates the download segment of the gui
    fn downloads(&self) -> Column<Message> {
        let sections = self.downloads.iter().map(download_entry);

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
    }
}

/// creates an entry for a single download in the list of downloads
fn download_entry(dl: &Download) -> Element<Message> {
    let check_icon = std::include_bytes!("../check.png");
    let alert_icon = std::include_bytes!("../alert.png");

    let download_state: Element<Message> = match &dl.state {
        DownloadState::Starting => text("Starting ...").into(),
        DownloadState::Downloading {
            downloaded_bytes,
            complete_size,
        } => {
            if let Some(size) = complete_size {
                progress_bar(0.0..=*size as f32, *downloaded_bytes as f32).into()
            } else {
                text(format!(
                    "{} downloaded (size unknown)",
                    format_size(*downloaded_bytes, BINARY)
                ))
                .into()
            }
        }
        DownloadState::Done { completed_size } => row!(
            image(image::Handle::from_memory(check_icon))
                .width(20)
                .height(20),
            text(format!(
                "Downloaded {}",
                format_size(*completed_size, BINARY)
            ))
        )
        .spacing(5)
        .align_items(Alignment::Center)
        .into(),
        DownloadState::Error { message } => row!(
            image(image::Handle::from_memory(alert_icon))
                .width(20)
                .height(20),
            text(message.replace('\n', " "))
        )
        .spacing(5)
        .align_items(Alignment::Center)
        .into(),
    };
    column!(text(&dl.url), download_state).spacing(5).into()
}

/// creates a container for the whole gui that adds some padding
fn main_container<'a, T>(content: T) -> Container<'a, Message>
where
    T: Into<Element<'a, Message, Theme, Renderer>>,
{
    container(content)
        .padding(Padding::from([30, 80]))
        .center_x()
        .center_y()
}

/// creates a new stream that executes the download
fn create_download_stream(link: String, id: usize) -> impl Stream<Item = Message> {
    let stream = try_stream! {
        let mut resp = reqwest::get(&link).await?;
        let storage_path = PathBuf::from(link.split('/').last().unwrap());
        if storage_path.exists(){
            Err(anyhow::anyhow!("File already exists: {}", storage_path.display()))?;
        }

        let mut writer = BufWriter::new(File::create(storage_path)?);
        yield Message::DownloadStarted { id, opt_size: resp.content_length() };
        while let Some(bytes) = resp.chunk().await? {
            writer.write_all(&bytes)?;
            yield Message::DownloadProgress { id, bytes: bytes.len() };
        }
        yield Message::DownloadFinished { id };
    };

    stream.map(
        move |msg_result: anyhow::Result<Message>| match msg_result {
            Ok(msg) => msg,
            Err(e) => Message::DownloadErrored {
                id,
                message: format!("{e:?}"),
            },
        },
    )
}

fn main() {
    Downloader::run(Settings::default()).unwrap()
}
