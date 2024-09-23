use iced::{Renderer, Theme};

use crate::frontend::iced::Message;

use super::container::container_selection_list;

pub fn selectionlist(
    elements: Vec<iced::Element<'_, Message, Theme, Renderer>>,
) -> iced::Element<Message> {
    let mut column = iced::widget::Column::new();
    for element in elements {
        column = column.push(element);
    }
    iced::widget::Container::new(column.align_x(iced::Alignment::Center).spacing(0))
        .style(container_selection_list)
        .into()
}
