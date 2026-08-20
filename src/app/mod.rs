mod components;
mod display_screen;
mod gallery_screen;
mod ingest_screen;
mod tag_screen;
mod title_screen;

use anyhow::{Result, anyhow};
use iced::event::{self, Event};
use iced::widget::operation;
use iced::{Element, Subscription, Task, Theme, keyboard, keyboard::key};
use tracing::{error, info};

use crate::db::models::tags::TagTreeData;
use crate::db::{self, DbPool};

use self::display_screen::Display;
use self::gallery_screen::Gallery;
use self::ingest_screen::Ingest;
use self::tag_screen::Tags;
use self::title_screen::Title;

pub struct App {
    screen: Screen,
    theme: Theme,
    db_pool: DbPool,
}

impl App {
    pub fn new(db_pool: DbPool) -> Self {
        let num_content =
            db::models::content::content_count(&db_pool).expect("Failed to query the database");
        Self {
            screen: Screen::Title(Title::new_with_num_content(Some(num_content))),
            theme: Theme::Ferra,
            db_pool,
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

#[derive(Debug)]
pub enum Message {
    Title(title_screen::Message),
    Tags(tag_screen::Message),
    LoadedTagTreeData(Result<TagTreeData>),
    Gallery(gallery_screen::Message),
    Ingest(ingest_screen::Message),
    Display(display_screen::Message),
    Event(Event),
}

pub fn run(db_pool: DbPool) -> iced::Result {
    iced::application(move || App::new(db_pool.clone()), update, view)
        .title("EmberVault")
        .theme(theme)
        .subscription(App::subscription)
        .run()
}

impl App {
    #[allow(clippy::unused_self)]
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
}

fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Title(msg) => {
            if let Screen::Title(title) = &mut state.screen {
                let action = title.update(msg);

                match action {
                    title_screen::Action::None => Task::none(),
                    title_screen::Action::Run(task) => task.map(Message::Title),
                    title_screen::Action::Tags => {
                        info!("Switching from the Title page to the Tags page");
                        state.screen = Screen::Tags(Tags::new());
                        load_tags(state.db_pool.clone())
                    }
                    title_screen::Action::Ingest => todo!(),
                    title_screen::Action::IngestDir => todo!(),
                    title_screen::Action::Search(_, _) => todo!(),
                }
            } else {
                Task::none()
            }
        }
        Message::Tags(msg) => {
            if let Screen::Tags(tags) = &mut state.screen {
                let action = tags.update(msg);

                match action {
                    tag_screen::Action::None => Task::none(),
                    tag_screen::Action::Run(task) => task.map(Message::Tags),
                    tag_screen::Action::Title => {
                        info!("Switching from the Tags page to the Title page");
                        let num_content = match db::models::content::content_count(&state.db_pool) {
                            Ok(num) => Some(num),
                            Err(err) => {
                                error!("Failed to get content count with error: {}", err);
                                None
                            }
                        };
                        state.screen = Screen::Title(Title::new_with_num_content(num_content));
                        Task::none()
                    }
                    tag_screen::Action::Search(_) => todo!(),
                    tag_screen::Action::AddTag(_) => todo!(),
                }
            } else {
                Task::none()
            }
        }
        Message::Gallery(_) => todo!(),
        Message::Ingest(_) => todo!(),
        Message::Display(_) => todo!(),
        Message::Event(event) => match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Tab),
                modifiers,
                ..
            }) => {
                if modifiers.shift() {
                    operation::focus_previous()
                } else {
                    operation::focus_next()
                }
            }
            _ => Task::none(),
        },
        Message::LoadedTagTreeData(tag_tree_data) => match tag_tree_data {
            Ok(data) => {
                if let Screen::Tags(tags) = &mut state.screen {
                    tags.set_tag_tree_data(data);
                }
                Task::none()
            }
            Err(_) => todo!(),
        },
    }
}

fn view(state: &App) -> Element<'_, Message> {
    match &state.screen {
        Screen::Title(title) => title.view().map(Message::Title),
        Screen::Tags(tags) => tags.view().map(Message::Tags),
        Screen::Gallery(gallery) => todo!(),
        Screen::Ingest(ingest) => todo!(),
        Screen::Display(display) => todo!(),
    }
}

fn theme(app: &App) -> Theme {
    app.theme.clone()
}

fn load_tags(pool: DbPool) -> Task<Message> {
    Task::perform(
        async move {
            tokio::task::spawn_blocking(move || db::models::tags::load_tag_tree_data(&pool))
                .await
                .map_err(|err| anyhow!(err))?
        },
        Message::LoadedTagTreeData,
    )
}
