use iced::Length::{Fill, FillPortion};
use iced::widget::{button, column, container, row, space, text, text_input};
use iced::{Element, Task, Theme};

#[derive(Default)]
pub struct Title {
    search_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tags,
    Ingest,
    IngestDir,
    Search,
    SearchContentChanged(String),
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
            .style(|_| container::primary(&Theme::Light))
            .center_x(Fill),
            // Center Elements
            container(
                column![
                    text("EmberVault").size(40).center().width(Fill),
                    space::vertical().height(50),
                    row![
                        space::horizontal().width(FillPortion(1)),
                        row![
                            text_input("Search with tags here...", &self.search_text)
                                .on_input(Message::SearchContentChanged)
                                .on_paste(Message::SearchContentChanged)
                                .on_submit(Message::Search)
                                .width(Fill),
                            button("Search").on_press(Message::Search)
                        ]
                        .width(FillPortion(3)),
                        space::horizontal().width(FillPortion(1))
                    ],
                    "CenterB"
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
