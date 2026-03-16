use iced::Length::{Fill, FillPortion};
use iced::widget::{Column, column, container, row, rule, space, text, toggler};
use iced::{Alignment, Element};
use strum::VariantArray;

use crate::types::search::order::{SearchOrder, SortOrderType};

#[derive(Debug, Default, Clone)]
pub struct State {
    ordering: SearchOrder,
    show_search_ordering: bool,
}

impl State {
    pub const fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ToggleSearchOrder(enabled) => {
                self.show_search_ordering = enabled;
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let row = row![
            rule::horizontal(2),
            toggler(self.show_search_ordering)
                .label("Search Order")
                .on_toggle(Message::ToggleSearchOrder),
            rule::horizontal(2)
        ]
        .align_y(Alignment::Center)
        .spacing(5)
        .width(Fill);
        if self.show_search_ordering {
            column![
                row,
                space::vertical().height(5),
                row![
                    text("Enabled").center().width(Fill),
                    text("Disabled").center().width(Fill)
                ],
                search_ordering_content(self.ordering.get_sort_order())
            ]
            .spacing(10)
            .into()
        } else {
            row.into()
        }
    }
}

fn search_ordering_content<'a>(sort_order: &[SortOrderType]) -> Element<'a, Message> {
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
    for item in SortOrderType::VARIANTS {
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

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleSearchOrder(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    None,
}
