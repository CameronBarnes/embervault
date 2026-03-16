mod search_ordering_widget;
mod search_widget;

use iced::Length::{Fill, FillPortion};
use iced::widget::{button, column, container, row, rule, space, text};
use iced::{Element, Padding, Task};

use crate::types::search;

#[derive(Default)]
pub struct Title {
    search_widget: search_widget::State,
    search_order_widget: search_ordering_widget::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tags,
    Ingest,
    IngestDir,
    Search(search_widget::Message),
    SearchOrder(search_ordering_widget::Message),
}

#[derive(Debug)]
pub enum Action {
    None,
    Run(Task<Message>),
    Tags,
    Ingest,
    IngestDir,
    Search(search::Options, String),
}

impl Title {
    pub fn new_with_num_content(num_content: usize) -> Self {
        let mut search_state = search_widget::State::default();
        search_state.num_content = num_content;
        Self {
            search_widget: search_state,
            search_order_widget: search_ordering_widget::State::default(),
        }
    }

    pub fn new(search_options: search::Options, num_content: usize) -> Self {
        Self {
            search_widget: search_widget::State::new(search_options, num_content),
            search_order_widget: search_ordering_widget::State::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Tags => Action::Tags,
            Message::Ingest => Action::Ingest,
            Message::IngestDir => Action::IngestDir,
            Message::Search(message) => match self.search_widget.update(message) {
                search_widget::Action::None => Action::None,
                search_widget::Action::Search(options, text) => Action::Search(options, text),
            },
            Message::SearchOrder(message) => match self.search_order_widget.update(message) {
                search_ordering_widget::Action::None => Action::None,
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(column![
            // Header
            container(header()).center_x(Fill),
            // Center Elements
            container(column![
                // Portion of the top half adds up to 5, 3 for spacer, 2 for title
                // As does the bottom half, 4 for content, 1 for spacer, which means that the top
                // of the bottom half should be just bellow exactly center, which is what we want
                space::vertical().height(FillPortion(3)),
                // Title
                container(text("EmberVault").size(48).center().width(Fill))
                    .padding(Padding::ZERO.bottom(50))
                    .height(FillPortion(2)),
                // Center components
                container(center_block(self)).height(FillPortion(4)),
                // Portion of the bottom half ads up to 5
                space::vertical().height(FillPortion(1)),
            ])
            .center(Fill),
            // Footer
            container(footer()).center_x(Fill),
        ])
        .center(Fill)
        .into()
    }
}

fn header<'a>() -> Element<'a, Message> {
    column![
        row![
            button("Tags").on_press(Message::Tags),
            space::horizontal(),
            button("Ingest").on_press(Message::Ingest),
            button("Ingest Dir").on_press(Message::IngestDir),
        ]
        .width(Fill),
        rule::horizontal(2)
    ]
    .into()
}

fn center_block(title: &Title) -> Element<'_, Message> {
    row![
        space::horizontal().width(FillPortion(1)),
        // Using an extra column here to make sure everything is centered horizontally
        column![
            title.search_widget.view().map(Message::Search),
            space::vertical().height(45),
            title.search_order_widget.view().map(Message::SearchOrder),
        ]
        .width(FillPortion(3)),
        space::horizontal().width(FillPortion(1))
    ]
    .into()
}

fn footer<'a>() -> Element<'a, Message> {
    column![rule::horizontal(2), row!["BottomL", "BottomC", "BottomR"]].into()
}
