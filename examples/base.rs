use iced::{Length, widget::image::Handle};
use magnifier::Magnifier;
struct Background {
    handle: Handle,
}

#[derive(Debug, Clone)]
enum Message {}

impl Background {
    fn view(&'_ self) -> iced::Element<'_, Message> {
        Magnifier::new(self.handle.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    fn new() -> Self {
        Self {
            handle: Handle::from_path("./misc/lulu.png"),
        }
    }
    fn update(&mut self, _message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }
}

fn main() -> iced::Result {
    iced::application(Background::new, Background::update, Background::view)
        .title("simple")
        .run()
}
