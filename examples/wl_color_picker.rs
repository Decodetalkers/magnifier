use std::collections::HashMap;

use iced::widget::image::Handle;
use iced::{Element, Length, Rectangle};
use iced_layershell::reexport::{Anchor, Layer, NewLayerShellSettings};
use iced_layershell::{
    daemon,
    settings::{LayerShellSettings, Settings, StartMode},
};
use iced_wayland_subscriber::{OutputInfo, WaylandEvent};
use libwayshot::OutputInfo as ShotOutputInfo;
use magnifier::{Magnifier, ScreenShot};
use rs_image::{GenericImageView, RgbaImage};
use wayland_client::Connection;
fn main() -> Result<(), iced_layershell::Error> {
    let connection = Connection::connect_to_env().unwrap();
    let connection2 = connection.clone();
    daemon(
        move || ColorPicker::new(connection.clone()),
        "osd",
        ColorPicker::update,
        ColorPicker::view,
    )
    .subscription(ColorPicker::subscription)
    .settings(Settings {
        layer_settings: LayerShellSettings {
            exclusive_zone: -1,
            anchor: Anchor::all(),
            start_mode: StartMode::Background,
            ..Default::default()
        },
        with_connection: Some(connection2.into()),
        ..Default::default()
    })
    .run()
}

struct ColorPicker {
    conn: Connection,
    wayshot: libwayshot::WayshotConnection,
    images: HashMap<iced::window::Id, RgbaImage>,
    handles: HashMap<iced::window::Id, Handle>,
}

#[iced_layershell::to_layer_message(multi)]
#[derive(Debug)]
enum Message {
    Wayland(WaylandEvent),
    WindowClose(iced::window::Id),
    OnSelected {
        id: iced::window::Id,
        screenshot: ScreenShot,
    },
}

impl ColorPicker {
    fn new(conn: Connection) -> Self {
        let wayshot = libwayshot::WayshotConnection::from_connection(conn.clone()).unwrap();
        Self {
            conn,
            wayshot,
            images: HashMap::new(),
            handles: HashMap::new(),
        }
    }
    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            iced_wayland_subscriber::listen(self.conn.clone()).map(Message::Wayland),
            iced::window::close_events().map(Message::WindowClose),
        ])
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Wayland(WaylandEvent::OutputInsert(OutputInfo {
                wl_output,
                name,
                description,
                ..
            })) => {
                let output_info = ShotOutputInfo {
                    wl_output: wl_output.clone(),
                    name,
                    description,
                    transform: libwayshot::reexport::Transform::Normal,
                    physical_size: libwayshot::Size {
                        width: 0,
                        height: 0,
                    },
                    logical_region: libwayshot::LogicalRegion::default(),
                };
                let image = self
                    .wayshot
                    .screenshot_single_output(&output_info, false)
                    .unwrap()
                    .to_rgba8();
                let (id, task) = Message::layershell_open(NewLayerShellSettings {
                    exclusive_zone: Some(-1),
                    anchor: Anchor::all(),
                    layer: Layer::Top,
                    ..Default::default()
                });

                let handle = Handle::from_rgba(image.width(), image.height(), image.to_vec());
                self.images.insert(id, image);
                self.handles.insert(id, handle);
                return task;
            }
            Message::WindowClose(id) => {
                self.images.remove(&id);
                self.handles.remove(&id);
            }
            Message::OnSelected {
                id,
                screenshot:
                    ScreenShot {
                        bounds: Rectangle { width, height, .. },
                        point,
                    },
            } => {
                let image = &self.images[&id];
                let image_width = image.width();
                let image_height = image.height();
                let point_x = (point.x * (image_width as f32) / width).round() as u32;
                let point_y = (point.y * (image_height as f32) / height).round() as u32;
                let [r, g, b, _] = image.view(point_x, point_y, 1, 1).get_pixel(0, 0).0;
                println!("RGB       : R:{r}, G:{g}, B:{b}");
                println!(
                    "RGB(float): R:{:.2}, G:{:.2}, B:{:.2}",
                    r as f32 / 255.,
                    g as f32 / 255.,
                    b as f32 / 255.,
                );
                println!("16hex      : #{:02x}{:02x}{:02x}", r, g, b);
                return iced_runtime::task::effect(iced_runtime::Action::Exit);
            }
            _ => {}
        }
        iced::Task::none()
    }
    fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        let handle = self.handles[&id].clone();
        Magnifier::new(handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .scale(3.)
            .on_selected(move |screenshot| Message::OnSelected { id, screenshot })
            .into()
    }
}
