use iced::widget::{button, column, container, row, space, text, toggler};
use iced::{Alignment, Border, Element, Length, Theme};

use super::{Settings, SettingsMessage};
use crate::data::settings::{AppScreen, AppTheme, RunMode, ShellKind, TerminalKind, TextSize};
use crate::utils::font_size::font_size;


impl Settings {
    pub fn setting_list_view(&self, text_size: &TextSize) -> Element<'_, SettingsMessage> {
        container(
            column![
                // Appearance
                Self::section_view(
                    "Appearance",

                    column![
                        Self::setting_row_view(
                            "Theme",
                            row![
                                Self::choice_button_view("Light", self.setting_list.theme == AppTheme::Light, SettingsMessage::SetThemeSetting(AppTheme::Light), text_size),
                                Self::choice_button_view("Dark", self.setting_list.theme == AppTheme::Dark, SettingsMessage::SetThemeSetting(AppTheme::Dark), text_size),
                            ]
                            .spacing(8)
                            .into(),
                            text_size
                        ),

                        Self::setting_row_view(
                            "Default screen",
                            row![
                                Self::choice_button_view("Home", self.setting_list.screen == AppScreen::Home, SettingsMessage::SetScreenSetting(AppScreen::Home), text_size),
                                Self::choice_button_view("Dashboard", self.setting_list.screen == AppScreen::Dashboard, SettingsMessage::SetScreenSetting(AppScreen::Dashboard), text_size),
                                Self::choice_button_view("Settings", self.setting_list.screen == AppScreen::Settings, SettingsMessage::SetScreenSetting(AppScreen::Settings), text_size),
                            ]
                            .spacing(8)
                            .into(),
                            text_size
                        ),

                        Self::setting_row_view(
                            "Text size",
                            row![
                                Self::choice_button_view("Small", self.setting_list.text_size == TextSize::Small, SettingsMessage::SetTextSize(TextSize::Small), text_size),
                                Self::choice_button_view("Normal", self.setting_list.text_size == TextSize::Normal, SettingsMessage::SetTextSize(TextSize::Normal), text_size),
                                Self::choice_button_view("Large", self.setting_list.text_size == TextSize::Large, SettingsMessage::SetTextSize(TextSize::Large), text_size),
                            ]
                            .spacing(8)
                            .into(),
                            text_size
                        ),
                    ]
                    .spacing(12)
                    .into(),

                    text_size
                ),

                // Execution
                Self::section_view(
                    "Execution",
                    column![
                        Self::setting_row_view(
                            "Shell",
                            row![
                                Self::choice_button_view("Bash", self.setting_list.shell == ShellKind::Bash, SettingsMessage::SetShell(ShellKind::Bash), text_size),
                            ]
                            .spacing(8)
                            .into(),
                            text_size
                        ),

                        Self::setting_row_view(
                            "Terminal",
                            row![
                                Self::choice_button_view("Gnome", self.setting_list.terminal == TerminalKind::Gnome, SettingsMessage::SetTerminal(TerminalKind::Gnome), text_size),
                                Self::choice_button_view("Konsole", self.setting_list.terminal == TerminalKind::Konsole, SettingsMessage::SetTerminal(TerminalKind::Konsole), text_size),
                                Self::choice_button_view("Kitty", self.setting_list.terminal == TerminalKind::Kitty, SettingsMessage::SetTerminal(TerminalKind::Kitty), text_size),
                                Self::choice_button_view("Xterm", self.setting_list.terminal == TerminalKind::Xterm, SettingsMessage::SetTerminal(TerminalKind::Xterm), text_size),
                            ]
                            .spacing(8)
                            .into(),
                            text_size
                        ),

                        Self::setting_row_view(
                            "On Run",
                            row![
                                Self::choice_button_view("New window", self.setting_list.run_mode == RunMode::NewWindow, SettingsMessage::SetRunMode(RunMode::NewWindow), text_size),
                                Self::choice_button_view("Reuse session", self.setting_list.run_mode == RunMode::ReuseSession, SettingsMessage::SetRunMode(RunMode::ReuseSession), text_size),
                            ]
                            .spacing(8)
                            .into(),
                            text_size
                        ),

                        Self::setting_row_view(
                            "Keep open",
                            toggler(self.setting_list.keep_open)
                                .on_toggle(SettingsMessage::ToggleKeepOpen)
                                .into(),
                            text_size
                        ),
                    ]
                    .spacing(12)
                    .into(),

                    text_size
                ),

                // Data
                Self::section_view(
                    "Data",
                    column![
                        Self::setting_row_view("Directory", text(self.data_dir.clone()).into(), text_size),
                        button("Open folder")
                            .on_press(SettingsMessage::OpenDataDir),
                    ]
                    .spacing(12)
                    .into(),
                    text_size
                ),

                // About
                Self::section_view(
                    "About",
                    column![
                        Self::setting_row_view("Version", text(self.version.clone()).into(), text_size),

                    ]
                    .spacing(12)
                    .into(),
                    text_size
                ),
            ]
            .spacing(14)
            .padding(20)
        )
        .width(800)
        .into()
    }

    fn section_view<'a: 'static>(
        title: &'a str,
        content: Element<'a, SettingsMessage>,
        text_size: &TextSize
    ) -> Element<'a, SettingsMessage> {
        container(
            column![
                text(title).size(font_size(18.0, text_size)),
                space().height(8),
                content,
            ]
            .spacing(6),
        )
        .padding(16)
        .width(Length::Fill)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary.scale_alpha(0.5),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn setting_row_view<'a>(
        label: &'a str,
        setting: Element<'a, SettingsMessage>,
        text_size: &TextSize
    ) -> Element<'a, SettingsMessage> {
        row![
            text(label)
                .size(font_size(15.0, text_size))
                .width(Length::Fixed(160.0)),
            setting,
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
    }

    fn choice_button_view<'a>(
        label: &'a str,
        active: bool,
        on_press: SettingsMessage,
        text_size: &TextSize
    ) -> Element<'a, SettingsMessage> {
        button(text(label).size(font_size(14.0, text_size)))
            .on_press(on_press)
            .padding([6, 12])
            .style(move |theme: &Theme, _| {
                let p = theme.palette(); // ISSUE: must be extended palette
                if active {
                    button::Style {
                        background: Some(p.primary.into()),
                        text_color: p.text,
                        border: Border {
                            color: p.primary,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                } else {
                    button::Style {
                        background: None,
                        text_color: p.text,
                        border: Border {
                            color: p.primary.scale_alpha(0.5),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }
            })
            .into()
    }
}
