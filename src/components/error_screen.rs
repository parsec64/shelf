use iced::widget::{center, column, text};
use iced::Element;

use crate::utils::font_size::sv;


/*
 * Shows a startup error (e.g. the database could not be opened) instead of loading forever.
 */
pub fn error_screen_view<'a, Message: 'a>(error: &'a str) -> Element<'a, Message> {
    center(
        column![
            text("Shelf could not start").size(sv(24.0)),
            text(error).size(sv(15.0)),
            text("See the terminal output for details.").size(sv(14.0)),
        ]
        .spacing(10)
    )
    .into()
}
