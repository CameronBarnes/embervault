use iced::widget::{button, text};
use iced::{Element, Theme};

pub struct App {
    counter: i32,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: Default::default(),
            theme: Theme::GruvboxDark,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
}

pub fn run() -> iced::Result {
    iced::application(App::default, update, view)
        .theme(theme)
        .run()
}

const fn update(counter: &mut App, message: Message) {
    match message {
        Message::IncrementPressed => counter.counter += 1,
    }
}

fn view(counter: &App) -> Element<'_, Message> {
    button(text(counter.counter))
        .on_press(Message::IncrementPressed)
        .into()
}

fn theme(app: &App) -> Theme {
    app.theme.clone()
}
