use iced::widget::{center, text};
use iced::Element;

use crate::data::settings::TextSize;
use crate::utils::font_size::font_size;


/*
 * Shows a loading message while page data are being loaded.
 */
pub fn loading_screen_view<'a, Message: 'a>(text_size: &TextSize) -> Element<'a, Message> {
    center(
        text("Loading...").size(font_size(15.0, text_size))
    )
    .into()
}
