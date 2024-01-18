use iced::widget::{image, Image, Row};
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
        let directory_path = "/Users/viktornagy/Pictures/german-cheatsheet";

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
        let mut images = vec![];
        for path in &self.paths {
            let image = Image::<image::Handle>::new(path)
                .width(Length::Fill)
                .height(Length::Fill);

            images.push(Element::new(image));
        }

        Row::with_children(images)
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }
}
