mod desktop_bus;
mod desktop_message_receiver;
mod desktop_message_transport;
mod message;

use chrono::Local;
use cosmic::{
    app::{self, Settings},
    cctk::sctk::shell::wlr_layer::{Anchor, Layer},
    executor,
    iced::{
        core::text::Alignment,
        runtime::platform_specific::wayland::{
            layer_surface::{IcedMargin, SctkLayerSurfaceSettings},
            CornerRadius,
        },
        time, window, Length, Padding, Subscription,
    },
    surface,
    widget::text,
    Core, Element,
};
use futures::stream::StreamExt;
use iana_time_zone::get_timezone;
use std::time::Duration;
use tokio_stream::wrappers::BroadcastStream;

use crate::{desktop_bus::init_dbus, desktop_message_transport::DBUS_TX, message::Message};

pub struct Clock {
    core: Core,
    is_visible: bool,
    layer_shell_id: window::Id,
}

impl app::Application for Clock {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    const APP_ID: &'static str = "com.utools.uclock";

    fn init(core: Core, _flags: ()) -> (Self, cosmic::Task<cosmic::Action<Message>>) {
        tokio::spawn(init_dbus());

        let layer_shell_id = window::Id::unique();

        let app = Clock {
            core,
            is_visible: false,
            layer_shell_id,
        };

        (app, cosmic::Task::none())
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            time::every(Duration::from_secs(1)).map(|_| Message::Tick),
            Subscription::run_with(std::any::TypeId::of::<Self>(), |_| {
                BroadcastStream::new(DBUS_TX.subscribe())
                    .filter_map(|result| async move { result.ok() })
            }),
        ])
    }

    fn update(&mut self, message: Message) -> cosmic::Task<cosmic::Action<Message>> {
        match message {
            Message::Toggle => {
                self.is_visible = !self.is_visible;
                if self.is_visible {
                    let id = self.layer_shell_id;
                    let layer_shell = surface::action::simple_layer_shell(
                        || surface::action::LiveSettings {
                            blur: Some(true),
                            corners: Some(CornerRadius {
                                top_left: 0,
                                top_right: 0,
                                bottom_right: 0,
                                bottom_left: 0,
                            }),
                            padding: Some(IcedMargin {
                                top: 0,
                                right: 0,
                                bottom: 0,
                                left: 0,
                            }),
                            ..Default::default()
                        },
                        move || SctkLayerSurfaceSettings {
                            id,
                            layer: Layer::Overlay,
                            anchor: Anchor::BOTTOM | Anchor::RIGHT,
                            size: Some((Some(200), Some(160))),
                            namespace: "uclock".to_string(),
                            margin: IcedMargin {
                                top: 0,
                                left: 0,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Some(move || -> Element<'static, cosmic::Action<Message>> {
                            let time = Local::now();
                            let time_formatted = time.format("%H:%M:%S").to_string();
                            let date_formatted = time.format("%B %d").to_string();

                            let timezone = get_timezone().unwrap();

                            let timezone_text = text(timezone);
                            let date_text = text(date_formatted);

                            cosmic::iced::widget::column([
                                timezone_text.size(12).align_x(Alignment::Left).into(),
                                text(time_formatted)
                                    .size(48)
                                    .align_x(Alignment::Center)
                                    .into(),
                                date_text
                                    .size(16)
                                    .align_x(Alignment::Right)
                                    .width(Length::Fill)
                                    .into(),
                            ])
                            .padding(Padding::new(10.0))
                            .into()
                        }),
                    );
                    return surface::surface_task(layer_shell);
                } else {
                    let destroy = surface::action::destroy_layer_shell(self.layer_shell_id);
                    return surface::surface_task(destroy);
                }
            }
            Message::Tick => (),
        }
        cosmic::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        cosmic::iced::widget::text("").into()
    }
}

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<Clock>(Settings::default().no_main_window(true), ())
}
