use crate::data::settings::TextSize;

pub fn font_size(base: f32, text_size: &TextSize) -> f32 {
    match text_size {
            TextSize::Small => base * 0.85,
            TextSize::Normal => base * 1.0,
            TextSize::Large => base * 1.15,
    }
}
