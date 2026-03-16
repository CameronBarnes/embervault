use std::time::Duration;

use iced::Length::Fill;
use iced::widget::{
    button, checkbox, column, container, lazy, row, space, text, text_input, tooltip,
};
use iced::{Element, Padding};
use strum::VariantArray;

use crate::types::{content, search};

#[derive(Debug, Clone, Default)]
pub struct State {
    search_options: search::Options,
    text: String,
    pub(super) num_content: usize,
}

impl State {
    pub const fn new(search_options: search::Options, num_content: usize) -> Self {
        Self {
            search_options,
            text: String::new(),
            num_content,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Search => Action::Search(self.search_options.clone(), self.text.clone()),
            Message::SearchContentChanged(text) => {
                self.text = text;
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
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            // Content count and Content/Pool search type
            container(content_count_and_search_type(
                self.num_content,
                *self.search_options.search_type()
            ))
            .padding(Padding::ZERO.horizontal(5)),
            // Search bar
            container(search_bar(
                &self.text,
                self.search_options.search_type().is_some(),
                self.search_options.has_any_allowed_content_types()
            )),
            container(filter_private_and_content_type(&self.search_options))
                .padding(Padding::ZERO.horizontal(5)),
        ]
        .spacing(15)
        .width(Fill)
        .into()
    }
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
    let text = match (allowed_search_content_pools, allowed_search_content_types) {
        (true, true) => {
            return bar.into();
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

#[derive(Clone, Debug)]
pub enum Message {
    Search,
    SearchContentChanged(String),
    ToggleContent(bool),
    TogglePools(bool),
    TogglePrivate(bool),
    ToggleContentType(content::Type, bool),
}

pub enum Action {
    None,
    Search(search::Options, String),
}
