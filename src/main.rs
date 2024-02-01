use iced::event::Event;
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::KeyCode;
use iced::widget::container::Appearance;
use iced::widget::image::Handle;
use iced::widget::{Checkbox, Column, Container, Image, Row};
use iced::{
    executor, subscription, theme, window, Alignment, Application, Color, Command, Element, Length,
    Settings, Subscription, Theme,
};
use std::{env, fs, process};

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
    offset: usize,
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
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            panic!("Please provide a directory as an argument");
        }
        let directory_path = args[1].clone();

        let paths: Vec<String> = fs::read_dir(directory_path)
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
            .collect();

        (
            Self {
                selected: vec![false; paths.len()],
                cursor: 0,
                offset: 0,
                paths,
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
                self.selected[id] = !value;
            }
            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyPressed { key_code, .. } = keyboard_event {
                        match key_code {
                            KeyCode::Q => {
                                // TODO: maybe gracefully die here
                                for (i, path) in self.paths.iter().enumerate() {
                                    if self.selected[i] {
                                        println!("{}", path);
                                    }
                                }
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
                            KeyCode::Space | KeyCode::M => {
                                self.selected[self.cursor + self.offset] =
                                    !self.selected[self.cursor + self.offset];
                            }

                            KeyCode::D => {
                                let path_to_delete = self.paths.remove(self.cursor + self.offset);
                                fs::remove_file(path_to_delete).unwrap();
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        if self.cursor as i32 + cursor_change < 0 || self.cursor as i32 + cursor_change > 8 {
            if self.cursor + self.offset >= self.paths.len() {
                return Command::none();
            }
            if cursor_change < 0 {
                if self.offset > 0 {
                    self.offset -= 3
                }
            } else {
                self.offset += 3
            }
        } else {
            let temp = self.cursor as i32 + cursor_change;
            if (temp as usize) < self.paths.len() && temp >= 0 {
                self.cursor = temp as usize;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut row1: Vec<Element<_>> = Vec::with_capacity(3);
        let mut row2: Vec<Element<_>> = Vec::with_capacity(3);
        let mut row3: Vec<Element<_>> = Vec::with_capacity(3);

        let mut i = 0;

        let skip_distance = self.offset;
        for path in self.paths.iter().skip(skip_distance) {
            if i >= 9 {
                break;
            }
            let image = Image::<Handle>::new(path)
                .width(Length::Fill)
                .height(Length::FillPortion(2));

            let element = Element::new(
                Column::with_children(vec![
                    Element::new(image),
                    Element::new(
                        Checkbox::new("", self.selected[i + skip_distance], move |_x| {
                            Message::CheckboxToggled(i, self.selected[i + skip_distance])
                        })
                        .style(theme::Checkbox::Secondary),
                    ),
                ])
                .align_items(Alignment::Center)
                .padding(5)
                .spacing(5)
                .width(Length::FillPortion(1)),
            );

            let border = match i {
                i if i == self.cursor => Color::from_rgb(0.04, 0.04, 0.04),
                _ => Color::TRANSPARENT,
            };

            let container = Element::new(
                Container::new(element)
                    .padding(5)
                    .width(Length::FillPortion(1))
                    .style(move |_t: &_| Appearance {
                        border_color: border,
                        border_width: 3.0,
                        border_radius: [5.0, 5.0, 5.0, 5.0].into(),
                        ..Default::default()
                    }),
            );

            match i {
                i if i < 3 => row1.push(container),
                i if i < 6 => row2.push(container),
                i if i < 9 => row3.push(container),
                _ => break,
            }
            i += 1;
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
