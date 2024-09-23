use iced::Theme;

pub fn svg_icon(data: &'static [u8]) -> iced::widget::Svg<'_, Theme> {
    iced::widget::svg(iced::widget::svg::Handle::from_memory(data))
        .width(iced::Length::Shrink)
        .height(20)
}

pub fn svg_plain(data: &'static [u8]) -> iced::widget::Svg<'_, Theme> {
    iced::widget::svg(iced::widget::svg::Handle::from_memory(data))
}
