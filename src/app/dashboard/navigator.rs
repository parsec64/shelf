use iced::{Alignment, Background, Border, Element, Length, Theme, color};
use iced::widget::{button, center_x, container, row, space, text, text_input};
use iced_phosphor::regular;

use super::{Dashboard, DashboardMessage};
use crate::data::settings::{AppScreen, AppTheme, TextSize};
use crate::utils::font_size::font_size;


impl Dashboard {
    pub fn navigator_view(&self, title: &'static str, theme: &AppTheme, text_size: &TextSize) -> Element<'_, DashboardMessage> {
        container(
            row![
                // App name
                button(
                    text(title)
                        .size(font_size(18.0, text_size)))
                        .style(|theme: &Theme, _| {
                            button::Style {
                                border: Border::default(),
                                text_color: theme.palette().text,
                                ..Default::default()
                            }
                        }
                )
                .padding([0, 15])
                .on_press(DashboardMessage::SetScreen(AppScreen::Home)),

                // Search bar
                center_x(
                    container(
                        row![
                            // Search button
                            button(regular::magnifying_glass().size(font_size(16.0, text_size)))
                                .on_press(DashboardMessage::Search)
                                .style(|theme, _| button::Style {
                                    text_color: theme.palette().primary,
                                    ..Default::default()
                                }),

                            // Input
                            text_input("Search", &self.search_query)
                                .on_input(|query| DashboardMessage::SetSearchQuery(Some(query)))
                                .on_submit(DashboardMessage::Search)
                                .size(font_size(16.0, text_size))
                                .style(|theme: &Theme, _| text_input::Style {
                                    background: Background::Color(theme.palette().background).scale_alpha(0.0),
                                    border: Border {
                                        ..Default::default()
                                    },
                                    icon: theme.palette().primary,
                                    placeholder: theme.palette().primary,
                                    value: theme.palette().text,
                                    selection: theme.palette().primary.scale_alpha(0.3),
                                }),

                            // Clear button
                            match self.search_query.as_str() {
                                "" => container(space()),
                                _ => container(button(regular::x().size(font_size(15.0, text_size)))
                                    .on_press(DashboardMessage::SetSearchQuery(None))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    })),
                            }
                        ]
                        .align_y(Alignment::Center)
                    )
                    .width(font_size(600.0, text_size))
                    .padding(2)
                    .align_y(Alignment::Center)
                    .style(|theme: &Theme| container::Style {
                        border: Border {
                            color: theme.palette().primary,
                            width: 1.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                )
                .width(Length::Fill)
                .padding(3),

                // Shortcuts
                row![
                    // Settings
                    button(regular::gear_six().size(font_size(20.0, text_size)))
                        .style(|theme, _| button::Style {
                            text_color: theme.palette().primary,
                            ..Default::default()
                        })
                        .on_press(DashboardMessage::SetScreen(AppScreen::Settings)),

                    // Switch theme
                    button(match theme {
                        AppTheme::Dark => regular::moon()
                            .size(font_size(20.0, text_size))
                            .color(color!(0xF1FA8C)),
                        AppTheme::Light => regular::sun()
                            .size(font_size(20.0, text_size))
                            .color(color!(0xE77B00)),
                    })
                    .style(|_, _| button::Style {
                        ..Default::default()
                    })
                    .on_press(DashboardMessage::SetTheme),
                ]
                .spacing(10),
            ]
            .align_y(Alignment::Center)
            .spacing(20)
        )
        .padding([5, 20])
        .style(|theme| container::Style {
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
