use iced::event::{self, Event};

use iced::keyboard;
use iced::keyboard::key;
use iced::time::Instant;
use iced::widget::{
    self, button, center, column, container, horizontal_space, mouse_area,
    opaque, pick_list, row, stack, text, text_input,
};
use iced::{window, Command};
use iced::{
    Alignment, Color, ContentFit, Degrees, Element, Length, Radians,
    Subscription, Theme,
};

pub fn main() -> iced::Result {
    iced::program("Modal - Iced", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

#[derive(Default)]
struct App {
    show_modal: bool,
    usename: String,
    password: String,
}

#[derive(Debug, Clone)]
enum Message {
    ShowModal,
    HideModal,
    UserName(String),
    Password(String),
    Submit,
    Event(Event),
}

//事件上报处理
impl App {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ShowModal => {
                self.show_modal = true;
                widget::focus_next()
            }
            Message::HideModal => {
                self.hide_modal();
                Command::none()
            }
            Message::UserName(usename) => {
                self.usename = usename;
                Command::none()
            }
            Message::Password(passwd) => {
                self.password = passwd;
                Command::none()
            }
            Message::Submit => {
                if !self.usename.is_empty() && self.password.is_empty() {
                    self.hide_modal();
                }
                Command::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    self.hide_modal();
                    Command::none()
                }
                _ => Command::none(),
            },
        }
    }
}

impl App {
    fn hide_modal(&mut self) {
        self.show_modal = false;
        self.usename.clear();
        self.password.clear();
    }
}
