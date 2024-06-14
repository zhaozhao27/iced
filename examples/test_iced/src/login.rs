//! login
use crate::{Message, Tab};
use iced::{
    advanced::{
        graphics::core::Element as core_Element,
        overlay::Element as overlay_Element,
    },
    alignment::{self, Horizontal, Vertical},
    event::{self, Event},
    keyboard::{self, key},
    time::Instant,
    widget::{
        self, button, column, container, horizontal_space, mouse_area,
        pick_list, row, text, text_input, Button, Column, Container, Row, Text,
        TextInput,
    },
    window, Alignment, Color, Command, ContentFit, Element, Length,
    Subscription, Theme,
};
use iced_aw::{TabLabel, Tabs};

#[derive(Default)]
pub struct LoginTab {
    ip: String,
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    Ip(String),
    UserName(String),
    Password(String),
    Submit,
    Event(Event),
    None,
    UsernameChanged,
}

impl LoginTab {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: LoginMessage) -> Command<LoginMessage> {
        match message {
            LoginMessage::Ip(ip) => {
                self.ip = ip;
                Command::none()
            }
            LoginMessage::UserName(username) => {
                self.username = username;
                Command::none()
            }
            LoginMessage::Password(passwd) => {
                self.password = passwd;
                Command::none()
            }
            LoginMessage::Submit => {
                if !self.username.is_empty() && self.password.is_empty() {
                    self.hide_modal();
                }
                Command::none()
            }
            LoginMessage::Event(event) => match event {
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
            _ => Command::none(),
        }
    }
}
impl Tab for LoginTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let signup: Element<'_, LoginMessage> = container(
            column![column![
                column![
                    text("IpAddress").size(12),
                    text_input("192.168.126.124", &self.ip)
                        .on_input(LoginMessage::Ip)
                        .on_submit(LoginMessage::Submit)
                        .padding(5),
                ]
                .spacing(5),
                column![
                    text("Username").size(12),
                    text_input("root", &self.username)
                        .on_input(LoginMessage::UserName)
                        .on_submit(LoginMessage::Submit)
                        .padding(5),
                ]
                .spacing(5),
                column![
                    text("Password").size(12),
                    text_input("", &self.password)
                        .on_input(LoginMessage::Password)
                        .on_submit(LoginMessage::Submit)
                        .secure(true)
                        .padding(5),
                ]
                .spacing(5),
                button("Submit").on_press(LoginMessage::Submit),
            ]
            .spacing(10)]
            .spacing(20),
        )
        .width(300)
        .padding(10)
        .into();

        signup.map(Message::Login)
    }
    /*
    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, LoginMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    Column::new()
                        .push(text("Ip").size(12))
                        .push(
                            text_input("192.168.126.124", &self.ip)
                                .on_input(LoginMessage::Ip)
                                .on_submit(LoginMessage::Submit)
                                .padding(5),
                        )
                        .spacing(5),
                )
                .push(
                    Column::new()
                        .push(text("Username").size(12))
                        .push(
                            text_input("root", &self.username)
                                .on_input(LoginMessage::UserName)
                                .on_submit(LoginMessage::Submit)
                                .padding(5),
                        )
                        .spacing(5),
                )
                .push(
                    Column::new()
                        .push(text("Password").size(12))
                        .push(
                            text_input("", &self.password)
                                .on_input(LoginMessage::Password)
                                .on_submit(LoginMessage::Submit)
                                .secure(true)
                                .padding(5),
                        )
                        .spacing(5),
                )
                .push(button("Submit").on_press(LoginMessage::Submit)),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();
        content.map(Message::Login)
    }
    */
}

impl LoginTab {
    fn hide_modal(&mut self) {
        self.username.clear();
        self.password.clear();
    }
}
