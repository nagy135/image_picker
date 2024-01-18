use iced::widget::image::Handle;
use iced::widget::{column, image, row, Column, Image, Row};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use std::fs;

pub fn main() -> iced::Result {
    Picker::run(Settings::default())
}

const ALLOWED_IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

struct Picker {
    paths: Vec<String>,
}

impl Sandbox for Picker {
    type Message = ();

    fn new() -> Self {
        let directory_path = "/Users/viktornagy/Pictures";
        // let directory_path = "/Users/viktornagy/Pictures/silicon";

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
        }
    }

    fn title(&self) -> String {
        String::from("Image picker")
    }

    fn update(&mut self, _message: Self::Message) {
        ()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut row1: Vec<Element<_>> = vec![];
        let mut row2: Vec<Element<_>> = vec![];
        let mut row3: Vec<Element<_>> = vec![];

        let mut i = 0;

        for path in &self.paths {
            i += 1;
            println!("i {}", i);
            let image = Image::<Handle>::new(path)
                .width(Length::Fill)
                .height(Length::Fill);

            println!("image {:?}", image);
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
