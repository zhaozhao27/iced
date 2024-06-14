//! This is an test_gui App based on iced .

mod board;
mod login;
use iced::{
    alignment::{self, Horizontal, Vertical},
    event::{self, Event},
    keyboard::{self, key},
    theme,
    time::Instant,
    widget::{
        self, button, column, container, horizontal_space, mouse_area,
        pick_list, row, text, text_input, Column, Container, Text,
    },
    window, Alignment, Application, Color, Command, ContentFit, Element,
    Length, Subscription, Theme,
};
use iced_aw::{style::TabBarStyles, TabLabel, Tabs};
use login::{LoginMessage, LoginTab};

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

/// main function
pub fn main() -> iced::Result {
    AppState::run(iced::Settings::default())
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum TabId {
    Login,
    Board,
}

enum AppState {
    Loading,
    Loaded(State),
}
struct State {
    active_tab: TabId,
    login_tab: LoginTab,
    board_tab: board::BoardTab, //todo
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(TabId),
    Login(login::LoginMessage),
    Board(board::BoardMessage),
    Loaded(Result<(), String>),
    Event(Event),
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for AppState {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Test_GUI-iced")
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (AppState::Loading, Command::perform(load(), Message::Loaded))
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            AppState::Loading => {
                if let Message::Loaded(_) = message {
                    *self = AppState::Loaded(State {
                        active_tab: TabId::Login,
                        login_tab: LoginTab::new(), //todo
                        board_tab: board::BoardTab::new(), //todo
                    });
                }
                Command::none()
            }
            AppState::Loaded(state) => match message {
                Message::TabSelected(selected) => {
                    state.active_tab = selected;
                    Command::none()
                }
                Message::Login(message) => {
                    let commamd = state.login_tab.update(message);
                    commamd.map(Message::Login)
                }
                Message::Board(message) => {
                    let command = state.board_tab.update(message);
                    command.map(Message::Board)
                }
                _ => Command::none(),
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self {
            AppState::Loading => container(
                text("Loading ...")
                    .horizontal_alignment(Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),

            AppState::Loaded(state) => {
                let position = iced_aw::TabBarPosition::Top;
                let theme = TabBarStyles::default();
                Tabs::new(Message::TabSelected)
                    .tab_icon_position(iced_aw::tabs::Position::Left)
                    .push(
                        TabId::Login,
                        state.login_tab.tab_label(),
                        state.login_tab.view(),
                    )
                    .push(
                        TabId::Board,
                        state.board_tab.tab_label(),
                        state.board_tab.view(),
                    )
                    .tab_bar_style(theme.clone())
                    .set_active_tab(&state.active_tab)
                    .tab_bar_position(iced_aw::TabBarPosition::Top)
                    .into()
            }
        }
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content())
            .align_items(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Message>;
}
