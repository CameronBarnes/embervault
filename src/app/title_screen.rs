use std::time::Duration;

use iced::Length::{Fill, FillPortion, Shrink};
use iced::widget::{
    Column, button, checkbox, column, container, lazy, row, rule, space, text, text_input, toggler,
    tooltip,
};
use iced::{Alignment, Element, Padding, Task};
use strum::VariantArray;

use crate::types::{content, search};

#[derive(Default)]
pub struct Title {
    search_options: search::Options,
    search_text: String,
    num_content: usize,
    show_search_ordering: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tags,
    Ingest,
    IngestDir,
    Search,
    SearchContentChanged(String),
    ToggleContent(bool),
    TogglePools(bool),
    TogglePrivate(bool),
    ToggleContentType(content::Type, bool),
    ToggleSearchOrder(bool),
    Drop(iced::Point, iced::Rectangle),
    HandleZones(Vec<(iced::advanced::widget::Id, iced::Rectangle)>),
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
        Self {
            search_options: search::Options::default(),
            search_text: String::default(),
            num_content,
            show_search_ordering: false,
        }
    }

    pub fn new(search_options: search::Options, num_content: usize) -> Self {
        Self {
            search_text: String::default(),
            search_options,
            num_content,
            show_search_ordering: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Tags => Action::Tags,
            Message::Ingest => Action::Ingest,
            Message::IngestDir => Action::IngestDir,
            Message::Search => {
                Action::Search(self.search_options.clone(), self.search_text.clone())
            }
            Message::SearchContentChanged(text) => {
                self.search_text = text;
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
            Message::TogglePrivate(private) => {
                self.search_options.set_allow_private(private);
                Action::None
            }
            Message::ToggleContentType(content_type, enabled) => {
                self.search_options
                    .set_content_type_status(content_type, enabled);
                Action::None
            }
            Message::ToggleSearchOrder(enabled) => {
                self.show_search_ordering = enabled;
                Action::None
            }
            Message::Drop(point, rectangle) => todo!(),
            Message::HandleZones(items) => todo!(),
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
            "TopC",
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
            // Content count and Content/Pool search type
            container(content_count_and_search_type(
                title.num_content,
                *title.search_options.search_type()
            ))
            .padding(Padding::ZERO.horizontal(5)),
            // Search bar
            container(search_bar(
                &title.search_text,
                title.search_options.search_type().is_some(),
                title.search_options.has_any_allowed_content_types()
            )),
            container(filter_private_and_content_type(&title.search_options))
                .padding(Padding::ZERO.horizontal(5)),
            space::vertical().height(30),
            container(search_ordering(
                &title.search_options.get_sort_order(),
                title.show_search_ordering
            ))
        ]
        .spacing(15)
        .width(FillPortion(3)),
        space::horizontal().width(FillPortion(1))
    ]
    .into()
}

fn content_count_and_search_type<'a>(
    num_content: usize,
    search_type: search::Type,
) -> Element<'a, Message> {
    row![
        row!["NumContent: ", text(num_content.to_string())],
        space::horizontal().width(Fill),
        tooltip(
            checkbox(search_type.content())
                .label("Content")
                .on_toggle(Message::ToggleContent),
            "Should Content (Image, Video, GIFs, etc) show up in results",
            tooltip::Position::Top
        )
        .delay(Duration::from_secs(1))
        .style(container::rounded_box),
        space::horizontal().width(10),
        tooltip(
            checkbox(search_type.pool())
                .label("Pools")
                .on_toggle(Message::TogglePools),
            "Should collections of content(Playlists, etc) show up in results",
            tooltip::Position::Top
        )
        .delay(Duration::from_secs(1))
        .style(container::rounded_box)
    ]
    .into()
}

fn search_bar<'a>(
    text: &str,
    allowed_search_content_pools: bool,
    allowed_search_content_types: bool,
) -> Element<'a, Message> {
    let allowed_search = allowed_search_content_pools && allowed_search_content_types;
    let bar = row![
        text_input("Search with tags here...", text)
            .on_input(Message::SearchContentChanged)
            .on_paste(Message::SearchContentChanged)
            .on_submit_maybe(allowed_search.then_some(Message::Search))
            .width(Fill),
        button("Search").on_press_maybe(allowed_search.then_some(Message::Search))
    ];
    if allowed_search_content_pools && allowed_search_content_types {
        bar.into()
    } else {
        let text = match (allowed_search_content_pools, allowed_search_content_types) {
            (true, true) => {
                unreachable!("Verified that we proceed above if both checks are allowed")
            }
            (true, false) => "Select a Content Type (Image, Video, GIF, etc) before you can search",
            (false, true) => "Select Content, Pools, or both to search.",
            (false, false) => {
                "You must select Content or Pools to determine the kind of search, and a Content Type such as Image, Video, Gif, etc to search for"
            }
        };
        tooltip(bar, text, tooltip::Position::FollowCursor)
            .style(container::rounded_box)
            .into()
    }
}

// TODO: maybe add search ordering
fn filter_private_and_content_type(search_options: &search::Options) -> Element<'_, Message> {
    row![
        tooltip(
            checkbox(search_options.allow_private())
                .label("Search Private")
                .on_toggle(Message::TogglePrivate),
            "Should Content marked as private show up in results",
            tooltip::Position::Top
        )
        .style(container::rounded_box)
        .delay(Duration::from_secs(1)),
        space::horizontal().width(Fill),
        lazy(
            search_options.get_allowed_content_types(),
            |allowed_content_types| content_type_toggles(allowed_content_types)
        )
    ]
    .into()
}

fn content_type_toggles<'a>(allowed_content_types: &[content::Type]) -> Element<'a, Message> {
    let mut row = row![];
    for item in content::Type::VARIANTS {
        row = row.push(
            checkbox(allowed_content_types.contains(item))
                .label(item.to_string())
                .on_toggle(|enabled| Message::ToggleContentType(*item, enabled)),
        );
    }
    row.spacing(10).into()
}

fn search_ordering<'a>(
    sort_order: &[search::SortOrderType],
    show_search_ordering: bool,
) -> Element<'a, Message> {
    let row = row![
        rule::horizontal(2),
        toggler(show_search_ordering)
            .label("Search Order")
            .on_toggle(Message::ToggleSearchOrder),
        rule::horizontal(2)
    ]
    .align_y(Alignment::Center)
    .spacing(5)
    .width(Fill);
    if show_search_ordering {
        column![
            row,
            space::vertical().height(5),
            row![
                text("Enabled").center().width(Fill),
                text("Disabled").center().width(Fill)
            ],
            search_ordering_content(sort_order)
        ]
        .spacing(10)
        .into()
    } else {
        row.into()
    }
}

fn search_ordering_content<'a>(sort_order: &[search::SortOrderType]) -> Element<'a, Message> {
    let mut enabled = Vec::<Element<'a, Message>>::new();
    for item in sort_order {
        enabled.push(
            container(text!("{}. {item}", enabled.len()))
                .style(container::bordered_box)
                .padding(5)
                .width(Fill)
                .into(),
        );
    }
    let mut disabled = Vec::<Element<'a, Message>>::new();
    for item in search::SortOrderType::VARIANTS {
        if sort_order.contains(item) {
            continue;
        }
        disabled.push(
            container(text(item.to_string()))
                .style(container::bordered_box)
                .padding(5)
                .width(Fill)
                .into(),
        );
    }
    row![
        container(
            Column::with_children(enabled)
                .padding(10)
                .spacing(10)
                .width(150)
        )
        .style(container::bordered_box)
        .width(Fill)
        .height(FillPortion(1)),
        container(
            Column::with_children(disabled)
                .padding(10)
                .spacing(10)
                .width(100)
        )
        .style(container::bordered_box)
        .width(Fill)
        .height(FillPortion(1))
    ]
    .into()
}

fn footer<'a>() -> Element<'a, Message> {
    column![rule::horizontal(2), row!["BottomL", "BottomC", "BottomR"]].into()
}
