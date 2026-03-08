use iced::Length::{Fill, FillPortion};
use iced::widget::{button, column, container, row, space, text, text_input, toggler};
use iced::{Element, Task, Theme};

use crate::types::search;

#[derive(Default)]
pub struct Title {
    search_options: search::Options,
    search_text: String,
    num_content: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tags,
    Ingest,
    IngestDir,
    Search,
    SearchContentChanged(String),
    UpdateContentCount(usize),
    ToggleContent(bool),
    TogglePools(bool),
}

#[derive(Debug)]
pub enum Action {
    None,
    Run(Task<Message>),
    Tags,
    Ingest,
    IngestDir,
    Search(String),
}

impl Title {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Tags => Action::Tags,
            Message::Ingest => Action::Ingest,
            Message::IngestDir => Action::IngestDir,
            Message::Search => Action::Search(self.search_text.clone()),
            Message::SearchContentChanged(text) => {
                self.search_text = text;
                Action::None
            }
            Message::UpdateContentCount(num) => {
                self.num_content = num;
                Action::None
            }
            Message::ToggleContent(enabled) => {
                self.search_options
                    .search_type_mut()
                    .update_content(enabled);
                Action::None
            }
            Message::TogglePools(enabled) => {
                self.search_options.search_type_mut().update_pool(enabled);
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(column![
            // Header
            container(
                row![
                    button("Tags").on_press(Message::Tags),
                    space::horizontal(),
                    "TopC",
                    space::horizontal(),
                    button("Ingest").on_press(Message::Ingest),
                    button("Ingest Dir").on_press(Message::IngestDir),
                ]
                .width(Fill)
            )
            .center_x(Fill),
            // Center Elements
            container(
                column![
                    // Title
                    text("EmberVault").size(48).center().width(Fill),
                    space::vertical().height(50),
                    // Center components
                    row![
                        space::horizontal().width(FillPortion(1)),
                        // Using an extra column here to make sure everything inside is lined up
                        // together and always the same width
                        column![
                            // Content count and Content/Pool search type
                            row![
                                "NumContent: ",
                                "0", // TODO: Actually get the number here
                                space::horizontal().width(Fill),
                                toggler(self.search_options.search_type().content())
                                    .label("Content")
                                    .on_toggle(Message::ToggleContent),
                                space::horizontal().width(10),
                                toggler(self.search_options.search_type().pool())
                                    .label("Pools")
                                    .on_toggle(Message::TogglePools)
                            ],
                            space::vertical().height(5),
                            // Search bar
                            row![
                                text_input("Search with tags here...", &self.search_text)
                                    .on_input(Message::SearchContentChanged)
                                    .on_paste(Message::SearchContentChanged)
                                    .on_submit_maybe(
                                        self.search_options
                                            .search_type()
                                            .is_some()
                                            .then_some(Message::Search)
                                    )
                                    .width(Fill),
                                button("Search").on_press_maybe(
                                    self.search_options
                                        .search_type()
                                        .is_some()
                                        .then_some(Message::Search)
                                )
                            ],
                            space::vertical().height(5),
                            // TODO: Content types to search for, and maybe search ordering
                            "CenterB"
                        ]
                        .width(FillPortion(3)),
                        space::horizontal().width(FillPortion(1))
                    ],
                ]
                .spacing(5)
            )
            .center(Fill),
            // Footer
            container(row!["BottomL", "BottomC", "BottomR"])
                .style(|_| container::primary(&Theme::GruvboxLight))
                .center_x(Fill),
        ])
        .center(Fill)
        .into()
    }
}
