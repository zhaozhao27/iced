use iced::{
    advanced::graphics::futures::backend::default,
    widget::{
        column, container, horizontal_space, pick_list, row, scrollable,
        vertical_space, Column, Container, Scrollable, Slider, Text,
    },
    Alignment, Command, Element, Length, Renderer, Theme,
};

use iced_aw::{
    direction::Horizontal, number_input, selection_list::SelectionList,
    style::NumberInputStyles, tab_bar::TabLabel, NumberInput,
    SelectionListStyles,
};

use crate::{Message, Tab};

#[derive(Debug, Clone)]
pub enum BoardMessage {
    BoardSelected(BoardList),
    SlotSelected(u8),
}
#[derive(Default)]
pub struct BoardTab {
    selected_board: Option<BoardList>,
    selected_slot: Option<u8>,
}

impl BoardTab {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: BoardMessage) -> Command<BoardMessage> {
        match message {
            BoardMessage::BoardSelected(board) => {
                self.selected_board = Some(board);
                Command::none()
            }
            BoardMessage::SlotSelected(slot_id) => {
                self.selected_slot = Some(slot_id);
                Command::none()
            }
        }
    }
}

impl Tab for BoardTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Board")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, self::Message> {
        let board_list = pick_list(
            &BoardList::ALL[..],
            self.selected_board,
            BoardMessage::BoardSelected,
        )
        .placeholder("Select a board")
        .width(Length::Shrink);
        let slot: NumberInput<'_, u8, BoardMessage, Theme, Renderer> =
            number_input(9, 10, BoardMessage::SlotSelected)
                .style(NumberInputStyles::Default)
                .step(1);

        let content: Element<'_, self::BoardMessage> = scrollable(
            container(
                row![
                    column!["The Board to Test", board_list,]
                        .width(Length::Shrink)
                        .align_items(Alignment::Start)
                        .spacing(10),
                    column!["Slot", slot,]
                        .width(Length::Shrink)
                        .align_items(Alignment::Start)
                        .spacing(10),
                ]
                .padding(5)
                .align_items(Alignment::Start)
                .spacing(20),
            )
            .align_y(iced::alignment::Vertical::Top),
        )
        .into();

        content.map(Message::Board)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoardList {
    #[default]
    DC11SCU,
    DC11P422,
    DC11P512,
    DC11P514,
    DC12P514,
    DC11P524,
    DC11P616,
    Other,
}

impl BoardList {
    const ALL: [BoardList; 8] = [
        BoardList::DC11SCU,
        BoardList::DC11P422,
        BoardList::DC11P512,
        BoardList::DC11P514,
        BoardList::DC12P514,
        BoardList::DC11P524,
        BoardList::DC11P616,
        BoardList::Other,
    ];
}

impl std::fmt::Display for BoardList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoardList::DC11SCU => "DC11SCU",
                BoardList::DC11P422 => "DC11P422",
                BoardList::DC11P512 => "DC11P512",
                BoardList::DC11P514 => "DC11P514",
                BoardList::DC12P514 => "DC12P514",
                BoardList::DC11P524 => "DC11P524",
                BoardList::DC11P616 => "DC11P616",
                BoardList::Other => "Other Board",
            }
        )
    }
}
