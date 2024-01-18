use iced::event::Event;
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::KeyCode;
use iced::widget::container::Appearance;
use iced::widget::image::Handle;
use iced::widget::{Checkbox, Column, Container, Image, Row};
use iced::{
    executor, subscription, window, Alignment, Application, Color, Command, Element, Length,
    Settings, Subscription, Theme,
};
use std::{fs, process};

const ALLOWED_IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

pub fn main() -> iced::Result {
    Picker::run(Settings {
        window: window::Settings {
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}
#[derive(Debug, Default)]
struct Picker {
    paths: Vec<String>,
    selected: Vec<bool>,
    cursor: usize,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    CheckboxToggled(usize, bool),
}

impl Application for Picker {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Picker, Command<Message>) {
        let directory_path = "/Users/viktornagy/Pictures";

        (
            Self {
                selected: vec![false; 9],
                cursor: 1,
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
        let mut cursor_change: i32 = 0;
        match message {
            Message::CheckboxToggled(id, value) => {
                self.selected[id - 1] = !value;
            }
            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyPressed { key_code, .. } = keyboard_event {
                        match key_code {
                            KeyCode::Q => {
                                // TODO: maybe gracefully die here
                                process::exit(0);
                            }
                            KeyCode::J => {
                                cursor_change = 3;
                            }
                            KeyCode::K => {
                                cursor_change = -3;
                            }
                            KeyCode::H => {
                                cursor_change = -1;
                            }
                            KeyCode::L => {
                                cursor_change = 1;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        self.cursor =
            (self.cursor as i32 + cursor_change).clamp(0, self.paths.len() as i32) as usize;
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut row1: Vec<Element<_>> = Vec::with_capacity(3);
        let mut row2: Vec<Element<_>> = Vec::with_capacity(3);
        let mut row3: Vec<Element<_>> = Vec::with_capacity(3);

        let mut i = 0;

        for path in &self.paths {
            i += 1;
            if i > 9 {
                break;
            }
            let image = Image::<Handle>::new(path)
                .width(Length::Fill)
                .height(Length::FillPortion(2));

            let element = Element::new(
                Column::with_children(vec![
                    Element::new(image),
                    Element::new(Checkbox::new("", self.selected[i - 1], move |_x| {
                        Message::CheckboxToggled(i, self.selected[i - 1])
                    })),
                ])
                .align_items(Alignment::Center)
                .spacing(10)
                .width(Length::FillPortion(1)),
            );

            let border = match i {
                i if i == self.cursor => Color::BLACK,
                _ => Color::TRANSPARENT,
            };

            let container = Element::new(
                Container::new(element)
                    .padding(3)
                    .width(Length::FillPortion(1))
                    .style(move |_t: &_| Appearance {
                        border_color: border,
                        border_width: 2.0,
                        ..Default::default()
                    }),
            );

            match i {
                i if i <= 3 => row1.push(container),
                i if i <= 6 => row2.push(container),
                i if i <= 9 => row3.push(container),
                _ => break,
            }
        }

        Column::with_children(vec![
            Element::new(
                Row::with_children(row1)
                    .spacing(10)
                    .height(Length::FillPortion(1)),
            ),
            Element::new(
                Row::with_children(row2)
                    .spacing(10)
                    .height(Length::FillPortion(1)),
            ),
            Element::new(
                Row::with_children(row3)
                    .spacing(10)
                    .height(Length::FillPortion(1)),
            ),
        ])
        .padding(20)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}
