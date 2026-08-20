use iced::Length::{Fill, FillPortion};
use iced::widget::{button, column, container, row, rule, scrollable, space, text, text_input};
use iced::{Element, Padding, Task};

use crate::db::models::tags::{TagRecord, TagTreeData};

pub struct Tags {
    search_text: String,
    add_tag_text: String,
    selected_tag_id: Option<i32>,
    tag_tree_data: Option<TagTreeData>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Title,
    Search,
    AddTag,
    SearchTextChanged(String),
    AddTagTextChanged(String),
}

#[derive(Debug)]
pub enum Action {
    None,
    Run(Task<Message>),
    Title,
    Search(String),
    AddTag(String),
}

impl Tags {
    pub const fn new() -> Self {
        Self {
            search_text: String::new(),
            add_tag_text: String::new(),
            selected_tag_id: None,
            tag_tree_data: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Title => Action::Title,
            Message::Search => Action::Search(self.search_text.clone()),
            Message::SearchTextChanged(text) => {
                self.search_text = text;
                Action::None
            }
            Message::AddTag => {
                if self.add_tag_text.is_empty()
                    || self.add_tag_text.chars().all(char::is_whitespace)
                {
                    self.add_tag_text = String::new();
                    Action::None
                } else {
                    Action::Search(self.add_tag_text.clone())
                }
            }
            Message::AddTagTextChanged(text) => {
                self.add_tag_text = text;
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(column!(
            container(header(self)).center_x(Fill),
            container(center_block(self)).center(Fill),
            container(footer()).center_x(Fill)
        ))
        .center(Fill)
        .into()
    }

    pub fn set_tag_tree_data(&mut self, data: TagTreeData) {
        self.tag_tree_data = Some(data);
    }
}

fn header<'a>(tags: &Tags) -> Element<'a, Message> {
    column![
        row![
            button("Home").on_press(Message::Title),
            space::horizontal().width(FillPortion(1)),
            text_input("Search for tags...", &tags.search_text)
                .on_input(Message::SearchTextChanged)
                .on_paste(Message::SearchTextChanged)
                .on_submit(Message::Search),
            space::horizontal().width(FillPortion(2))
        ]
        .width(Fill),
        rule::horizontal(2)
    ]
    .into()
}

fn center_block(tags: &Tags) -> Element<'_, Message> {
    let tag: Option<TagRecord> = None; // TODO: Actually pull the selected tag once that works
    let tag_name = tag
        .as_ref()
        .map_or_else(|| String::from("None Selected"), |tag| tag.name.clone());
    column![
        row![
            column![
                row!["Tags", rule::horizontal(2)],
                container(scrollable("test")).center(Fill)
            ]
            .height(Fill),
            column![
                row!["Tag: ", text(tag_name), rule::horizontal(2)],
                container(scrollable("test2")).center(Fill)
            ]
            .height(Fill)
        ],
        row![
            space::horizontal().width(FillPortion(1)),
            container(row![
                text_input("Add New Tag", &tags.add_tag_text)
                    .on_input(Message::AddTagTextChanged)
                    .on_paste(Message::AddTagTextChanged)
                    .on_submit_maybe(tags.add_tag_text.is_empty().then_some(Message::AddTag)),
                button("Add Tag")
                    .on_press_maybe(tags.add_tag_text.is_empty().then_some(Message::AddTag))
            ]),
            space::horizontal().width(FillPortion(1))
        ]
        .width(Fill)
    ]
    .into()
}

fn footer<'a>() -> Element<'a, Message> {
    column![rule::horizontal(2), row!["BottomL", "BottomC", "BottomR"]].into()
}
