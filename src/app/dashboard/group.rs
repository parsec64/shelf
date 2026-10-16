use iced::font::{Family, Style};
use iced::widget::text::{Span, Wrapping};
use iced::widget::{Column, Row, button, center, center_x, column, container, mouse_area, rich_text, row, scrollable, space, text};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Theme, color};
use iced_phosphor::{fill, regular};

use crate::app::dashboard::{Dashboard, DashboardMessage, ItemFilter, ItemInfo, ItemType};
use crate::utils::font_size::{sv, sv_16, sv_20};
use crate::utils::formatting::clamp_name;


impl Dashboard {
    pub fn group_view(&self) -> Element<'_, DashboardMessage> {
        container(
            column![
                // Fancy group name
                // ISSUE: Green color doesn't match theme
                rich_text![
                    Span::<&str>::new("user@shelf").color(color!(0x50FA7B)),
                    Span::new(":"),
                    Span::new("~/").color(color!(0x276CF5)),
                    Span::new(self.group_selected.name.clone()).color(color!(0x276CF5)),
                    Span::new("$ "),
                ]
                .size(sv_16())
                .wrapping(Wrapping::WordOrGlyph),

                // Group controls and filters
                row![
                    // Filters
                    row![
                        // Command filter
                        button(match self.item_filter.command {
                            true => fill::terminal(),
                            false => regular::terminal(),
                        }
                        .size(sv_20()))
                        .padding([0, 3])
                        .on_press(DashboardMessage::SetFilter(ItemFilter {
                            command: !self.item_filter.command,
                            ..self.item_filter
                        }))
                        .style(|theme, _| button::Style {
                            background: None,
                            text_color: theme.palette().primary,
                            ..Default::default()
                        }),

                        // Script filter
                        button(match self.item_filter.script {
                            true => fill::code(),
                            false => regular::code(),
                        }
                        .size(sv_20()))
                        .padding([0, 3])
                        .on_press(DashboardMessage::SetFilter(ItemFilter {
                            script: !self.item_filter.script,
                            ..self.item_filter
                        }))
                        .style(|theme, _| button::Style {
                            background: None,
                            text_color: theme.palette().primary,
                            ..Default::default()
                        }),

                        // Alphabetical filter
                        button(match self.item_filter.alphabetical {
                            true => fill::funnel_simple(),
                            false => regular::funnel_simple(),
                        }
                        .size(sv_20()))
                        .padding([0, 3])
                        .on_press(DashboardMessage::SetFilter(ItemFilter {
                            alphabetical: !self.item_filter.alphabetical,
                            ..self.item_filter
                        }))
                        .style(|theme, _| button::Style {
                            background: None,
                            text_color: theme.palette().primary,
                            ..Default::default()
                        })
                    ]
                    .spacing(15),

                    space()
                        .width(Length::Fill),

                    // Edit group, Add command/script
                    if self.group_selected.name.as_str() != "Favourites"
                    && self.group_selected.name.as_str() != "Recent" {
                        row![
                            // Edit group
                            match self.group_selected.name.as_str() != "Default" {
                                true => container(
                                    button(center(fill::pencil_simple().size(sv_20())))
                                        .on_press(DashboardMessage::OpenEditGroupBox(self.group_selected.clone()))
                                        .height(Length::Shrink)
                                        .width(Length::Shrink)
                                        .padding(5)
                                        .style(|theme, _| button::Style {
                                            text_color: theme.palette().primary,
                                            ..Default::default()
                                        })
                                ),
                                false => container(space()),
                            },

                            space()
                                .width(Length::Fixed(20.0)),

                            // Add new command
                            button(
                                row![
                                    regular::plus()
                                        .size(sv_20())
                                        .style(|theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),

                                    text("Command")
                                        .size(sv_16())
                                        .style(|theme: &Theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),
                                ]
                                .align_y(Alignment::Center)
                                .spacing(10)
                            )
                            .on_press(DashboardMessage::OpenNewItemBox(ItemType::Command))
                            .style(|theme, _| button::Style {
                                border: Border {
                                    color: theme.palette().success,
                                    width: 2.0,
                                    radius: 4.into(),
                                },
                                ..Default::default()
                            }),

                            // Space
                            space()
                                .width(Length::Fixed(10.0)),

                            // Add new script
                            button(
                                row![
                                    regular::plus()
                                        .size(sv_20())
                                        .style(|theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),

                                    text("Script")
                                        .size(sv_16())
                                        .style(|theme: &Theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),
                                ]
                                .align_y(Alignment::Center)
                                .spacing(10)
                            )
                            .on_press(DashboardMessage::OpenNewItemBox(ItemType::Script))
                            .style(|theme, _| button::Style {
                                border: Border {
                                    color: theme.palette().success,
                                    width: 2.0,
                                    radius: 4.into(),
                                },
                                ..Default::default()
                            }),
                        ]
                    } else {
                        row![
                            space().height(36) // ISSUE: Guessed height for empty buttons
                        ]
                    }
                ],

                // Group description
                row![
                    // Description
                    container(
                        row![
                            // Text
                            container(match self.group_selected.description.as_ref() {
                                "" => text("No description")
                                    .size(sv_16())
                                    .font(Font {
                                        family: Family::Monospace,
                                        style: Style::Italic, // ISSUE: Italic not working
                                        ..Default::default()
                                    })
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().primary.scale_alpha(0.5)),
                                    }),
                                description => match self.group_description_open {
                                    true => text(description),
                                    false => text(clamp_name(description, 120)),
                                }
                                .size(sv_16())
                                .style(|theme: &Theme| text::Style {
                                    color: Some(theme.palette().primary),
                                }),
                            })
                            .width(Length::Fill),

                            // Collapse button
                            match self.group_selected.description.as_str() {
                                "" => container(space()),
                                _ => container(button(match self.group_description_open {
                                        true => regular::caret_up().size(sv_20()),
                                        false => regular::caret_down().size(sv_20()),
                                    })
                                    .on_press(DashboardMessage::SetGroupDescriptionOpen)
                                    .padding([0, 5])
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    })),
                            }
                        ]
                    )
                    .height(Length::Shrink)
                    .padding(10)
                    .clip(true)
                    .style(|theme| container::Style {
                        background: Some(Background::Color(theme.palette().primary.scale_alpha(0.1))),
                        ..Default::default()
                    }),
                ],

                // Item list
                scrollable(
                    center_x(
                        self.item_list
                            .iter()
                            .fold(
                                Column::new()
                                    .spacing(10),
                                |column, item| column.push(Self::item_card_view(item)),
                            )
                    )
                )
            ]
            .spacing(15)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(10.0)
        .style(|theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn item_card_view<'a>(item: &'a ItemInfo) -> Element<'a, DashboardMessage> {
        let (id, name, content, description, is_favourite, tags, highlighted, hovered, copied, _description_collapsed) = (
            item.id,
            match &item.name {
                Some(name) => name.as_str(),
                None => "No name", // never happens
            },
            item.content.as_str(),
            item.description.as_str(),
            item.is_favourite,
            item.tags.iter().collect::<Vec<_>>(),
            item.highlighted,
            item.hovered,
            item.copied,
            item.description_open,
        );

        let icon = match copied {
            true => regular::copy(),
            false => match item.item_type {
                ItemType::Command => regular::terminal(),
                ItemType::Script => regular::code(),
            },
        };

        fn tag_view(tag: &str) -> Element<'_, DashboardMessage> {
            container(
                text(tag)
                    .size(sv(13.0))
                    .color(color!(0xF8F8F2))
            )
            .padding([2, 5])
            .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(theme.palette().primary.scale_alpha(0.6))),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        }

        mouse_area(
            container(
                row![
                    // Icon, Content, Description and Tags
                    // Clicking anywhere copies the item content
                    button(
                        column![
                            // Icon, Name
                            row![
                                // Icon
                                icon
                                    .size(sv_20())
                                    .color(Color::from_rgb(0.3, 0.4, 0.6)),

                                // Name of script / Content of command TODO: highlight this
                                text(match item.item_type {
                                    ItemType::Command => content,
                                    ItemType::Script => name,
                                })
                                .wrapping(Wrapping::WordOrGlyph)
                                .size(sv(15.0))
                                .style(|theme: &Theme| text::Style {
                                    color: Some(theme.palette().text),
                                    ..Default::default()
                                }),

                                space().width(Length::Fill),
                            ]
                            .width(Length::Fill)
                            .spacing(8)
                            .align_y(Alignment::Center),

                            // Content of script
                            match item.item_type {
                                ItemType::Script => container(
                                    text(content)
                                    .size(sv(15.0))
                                    .color(Color::from_rgb(0.5, 0.5, 0.6))
                                )
                                .padding(5),
                                ItemType::Command => container(space()),
                            },

                            // Description
                            container(match description {
                                "" => text("No description")
                                    .size(sv(15.0))
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().primary.scale_alpha(0.5)),
                                        ..Default::default()
                                    }),
                                _ => text(description)
                                    .size(sv(15.0))
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().primary),
                                        ..Default::default()
                                    }),
                            })
                            .padding(5),

                            // Tags
                            tags
                                .iter()
                                .fold(
                                    Row::new().padding(5).spacing(6),
                                    |row, tag| row.push(tag_view(tag)),
                                ),
                        ]
                        .spacing(8)
                    )
                    .on_press(DashboardMessage::CopyItemContent(
                        id,

                    ))
                    .style(|_, _| button::Style {
                        ..Default::default()
                    }),

                    // Favourite, Edit, Delete buttons
                    column![
                        // Run
                        button(fill::play().size(sv_16()))
                            .on_press(DashboardMessage::RunCommand(content.to_string()))
                            .style(|theme, _| button::Style {
                                text_color: theme.palette().success,
                                ..Default::default()
                            }),

                        // Favourite
                        match is_favourite {
                            // If the item is a favourite, show the star icon
                            true => container(
                                button(fill::star().size(sv_16()))
                                    .on_press(DashboardMessage::SetItemFavourite(id))
                                    .style(|theme, _| button::Style {
                                        text_color: match theme.extended_palette().is_dark {
                                            true => color!(0xF1FA8C),
                                            false => color!(0xE77B00),
                                        },
                                        ..Default::default()
                                    })
                            ),
                            // If the item is not a favourite, hide the star icon but show the empty star icon on hover
                            false => match hovered {
                                true => container(
                                    button(regular::star().size(sv_16()))
                                        .on_press(DashboardMessage::SetItemFavourite(id))
                                        .style(|theme, _| button::Style {
                                            text_color: theme.palette().primary,
                                            ..Default::default()
                                        })
                                ),
                                false => container(space()),
                            },
                        },

                        // Edit, Delete
                        // Show the edit, delete buttons on hover
                        match hovered {
                            true => column![
                                button(fill::pencil_simple().size(sv_16()))
                                    .on_press(DashboardMessage::OpenEditItemBox(item.clone()))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    }),

                                button(fill::trash().size(sv_16()))
                                    .on_press(DashboardMessage::OpenDeleteItemBox(item.clone()))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().danger,
                                        ..Default::default()
                                    })
                            ],
                            false => column![],
                        }
                        .height(Length::Shrink)
                        .width(35)
                        .spacing(10),
                    ]
                    .height(sv(155.0)) // ISSUE: Approximate height that fits visible icon set
                    .spacing(10)
                ]
            )
            .width(800)
            .height(Length::Shrink)
            .padding(12)
            .style(move |theme| container::Style {
                border: Border {
                    color: match highlighted {
                        true => theme.palette().warning,
                        false => theme.palette().primary
                    },
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            })
        )
        .on_enter(DashboardMessage::SetItemHovered(id))
        .on_exit(DashboardMessage::SetItemHovered(id))
        .into()
    }
}
