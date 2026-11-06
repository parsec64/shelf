use iced::widget::{center, column, text};
use iced::Element;

use crate::data::settings::TextSize;
use crate::utils::font_size::font_size;


/*
 * Shows a startup error (e.g. the database could not be opened) instead of loading forever.
 */
pub fn error_screen_view<'a, Message: 'a>(error: &'a str, text_size: &TextSize) -> Element<'a, Message> {
    center(
        column![
            text("Shelf could not start").size(font_size(24.0, text_size)),
            text(error).size(font_size(15.0, text_size)),
            text("See the terminal output for details.").size(font_size(14.0, text_size)),
        ]
        .spacing(10)
    )
    .into()
}
