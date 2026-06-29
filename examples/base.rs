use iced::Rectangle;
use iced::{Length, widget::image::Handle};
use magnifier::Magnifier;
use magnifier::ScreenShot;
use rs_image::{GenericImageView, RgbImage};
struct Background {
    handle: Handle,
    image: RgbImage,
}

#[derive(Debug, Clone)]
enum Message {
    OnSelected(ScreenShot),
}

impl Background {
    fn view(&'_ self) -> iced::Element<'_, Message> {
        Magnifier::new(self.handle.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .on_selected(Message::OnSelected)
            .into()
    }
    fn new() -> Self {
        let image = rs_image::open("./misc/lulu.png").unwrap().to_rgb8();
        Self {
            handle: Handle::from_path("./misc/lulu.png"),
            image,
        }
    }
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        let Message::OnSelected(ScreenShot {
            bounds: Rectangle { width, height, .. },
            point,
        }) = message;

        let image_width = self.image.width();
        let image_height = self.image.height();
        let point_x = (point.x * (image_width as f32) / width).round() as u32;
        let point_y = (point.y * (image_height as f32) / height).round() as u32;
        let [r, g, b] = self.image.view(point_x, point_y, 1, 1).get_pixel(0, 0).0;
        println!("RGB       : R:{r}, G:{g}, B:{b}");
        println!(
            "RGB(float): R:{:.2}, G:{:.2}, B:{:.2}",
            r as f32 / 255.,
            g as f32 / 255.,
            b as f32 / 255.,
        );
        println!("16hex      : #{:02x}{:02x}{:02x}", r, g, b);

        iced::Task::none()
    }
}

fn main() -> iced::Result {
    iced::application(Background::new, Background::update, Background::view)
        .title("simple")
        .run()
}
