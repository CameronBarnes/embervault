mod components;
mod display_screen;
mod gallery_screen;
mod ingest_screen;
mod tag_screen;
mod title_screen;

use iced::{Element, Task, Theme};

use self::display_screen::Display;
use self::gallery_screen::Gallery;
use self::ingest_screen::Ingest;
use self::tag_screen::Tags;
use self::title_screen::Title;

pub struct App {
    screen: Screen,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::default(),
            theme: Theme::Ferra,
        }
    }
}

enum Screen {
    Title(Title),
    Tags(Tags),
    Gallery(Gallery),
    Ingest(Ingest),
    Display(Display),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Title(Title::default())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Title(title_screen::Message),
    Tags(tag_screen::Message),
    Gallery(gallery_screen::Message),
    Ingest(ingest_screen::Message),
    Display(display_screen::Message),
}

pub fn run() -> iced::Result {
    iced::application(App::default, update, view)
        .title("EmberVault")
        .theme(theme)
        .run()
}

fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Title(msg) => {
            if let Screen::Title(title) = &mut state.screen {
                let action = title.update(msg);

                match action {
                    title_screen::Action::None => Task::none(),
                    title_screen::Action::Run(task) => task.map(Message::Title),
                    title_screen::Action::Tags => todo!(),
                    title_screen::Action::Ingest => todo!(),
                    title_screen::Action::IngestDir => todo!(),
                    title_screen::Action::Search(_) => todo!(),
                }
            } else {
                Task::none()
            }
        }
        Message::Tags(_) => todo!(),
        Message::Gallery(_) => todo!(),
        Message::Ingest(_) => todo!(),
        Message::Display(_) => todo!(),
    }
}

fn view(state: &App) -> Element<'_, Message> {
    match &state.screen {
        Screen::Title(title) => title.view().map(Message::Title),
        Screen::Tags(tags) => todo!(),
        Screen::Gallery(gallery) => todo!(),
        Screen::Ingest(ingest) => todo!(),
        Screen::Display(display) => todo!(),
    }
}

fn theme(app: &App) -> Theme {
    app.theme.clone()
}
