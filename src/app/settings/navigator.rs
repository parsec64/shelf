use iced::widget::{button, container, row, text};
use iced::{Alignment, Border, Element, Length, Theme};
use iced_phosphor::regular;

use super::{Settings, SettingsMessage};
use crate::data::settings::{AppScreen, TextSize};
use crate::utils::font_size::font_size;


impl Settings {
    pub fn navigator_view(&self, text_size: &TextSize) -> Element<'static, SettingsMessage> {
        container(
            row![
                // Back button
                button(regular::caret_left().size(font_size(20.0, text_size)))
                    .padding(2.5)
                    .style(|theme: &Theme, _| {
                        button::Style {
                            text_color: theme.palette().text,
                            ..Default::default()
                        }
                    })
                    .on_press(SettingsMessage::SetScreen(AppScreen::Dashboard)),

                // Title
                text("Settings")
                    .size(font_size(18.0, text_size))
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.palette().text),
                            ..Default::default()
                        }
                    }),
            ]
            .padding(10)
            .spacing(20)
            .align_y(Alignment::Center),
        )
        .padding([0, 20])
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
    }
}
