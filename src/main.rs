use iced::event::Event;
use iced::keyboard::Event::KeyReleased;
use iced::keyboard::KeyCode;
use iced::widget::image::Handle;
use iced::widget::{column, Image, Row};
use iced::{
    executor, subscription, window, Alignment, Application, Command, Element, Length, Settings,
    Subscription, Theme,
};
use std::{fs, process};

const ALLOWED_IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

pub fn main() -> iced::Result {
    Picker::run(Settings {
        window: window::Settings {
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
#[derive(Debug, Default)]
struct Picker {
    paths: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
}

impl Application for Picker {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Picker, Command<Message>) {
        let directory_path = "/Users/viktornagy/Pictures";
        // let directory_path = "/Users/viktornagy/Pictures/silicon";

        (
            Self {
                paths: fs::read_dir(directory_path)
                    .unwrap()
                    .map(|r| r.unwrap().path().to_str().unwrap().to_string())
                    .filter(|p| {
                        for extension in &ALLOWED_IMAGE_EXTENSIONS {
                            if p.ends_with(extension) {
                                return true;
                            }
                        }
                        return false;
                    })
                    .collect(),
            },
            Command::none(),
        )
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn title(&self) -> String {
        String::from("Image picker")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyReleased { key_code, .. } = keyboard_event {
                        match key_code {
                            KeyCode::Q => {
                                process::exit(0);
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut row1: Vec<Element<_>> = vec![];
        let mut row2: Vec<Element<_>> = vec![];
        let mut row3: Vec<Element<_>> = vec![];

        let mut i = 0;

        for path in &self.paths {
            i += 1;
            let image = Image::<Handle>::new(path)
                .width(Length::Fill)
                .height(Length::Fill);

            match i {
                i if i <= 3 => row1.push(Element::new(image)),
                i if i <= 6 => row2.push(Element::new(image)),
                i if i <= 9 => row3.push(Element::new(image)),
                _ => break,
            }
        }

        column![
            Row::with_children(row1).height(Length::FillPortion(1)),
            Row::with_children(row2).height(Length::FillPortion(1)),
            Row::with_children(row3).height(Length::FillPortion(1)),
        ]
        .padding(20)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}
