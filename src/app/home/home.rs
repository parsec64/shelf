use iced::{Element, Task};
use iced::widget::{button, center, column, text};

use super::{Home, HomeMessage};
use crate::data::settings::{AppScreen, TextSize};
use crate::utils::font_size::font_size;


impl Home {
    pub fn new() -> (Self, Task<HomeMessage>) {
        (
            Self {},
            Task::none(),
        )
    }

    pub fn view(&self, text_size: &TextSize) -> Element<'_, HomeMessage> {
        center(
            column![
                text("Shelf").size(font_size(30.0, text_size)),

                button(text("Explore the shelf").size(font_size(20.0, text_size)))
                    .on_press(HomeMessage::SetScreen(AppScreen::Dashboard)),
            ]
            .spacing(10)
        )
        .into()
    }

    pub fn update(&mut self, message: HomeMessage, screen: &mut AppScreen) -> Task<HomeMessage> {
        match message {
            HomeMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
        }
    }
}
