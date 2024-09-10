use iced::{color, font, theme::Button, widget::Space};
use iced_aw::SelectionListStyles;

use crate::{
    backend::utils::usb_utils::UsbDevice,
    frontend::{
        iced::{wrap, LocationSelection, Message, Raspirus},
        theme::{
            button::{RaspirusButtonOrange, RaspirusButtonPrimary, RaspirusButtonSecondary},
            selection_list::RaspirusSelectionList,
        },
    },
};

impl Raspirus {
    pub fn main_menu<'a>(
        &'a self,
        expanded_language: bool,
        expanded_location: bool,
        expanded_usb: bool,
        selection: LocationSelection,
        usbs: &'a [UsbDevice],
    ) -> iced::Element<Message> {
        let top_row = iced::widget::Row::new()
            // language selection
            .push(iced_aw::widgets::DropDown::new(
                // button to trigger dropdown
                iced::widget::Row::new().push(
                    iced::widget::Button::new(iced::widget::Text::new(&self.language))
                        .on_press(Message::ToggleLanguageSelection),
                ),
                // dropdown selection list
                iced_aw::widgets::SelectionList::new(
                    &crate::SUPPORTED_LANGUAGES,
                    |_idx: usize, language: String| Message::LanguageChanged { language },
                )
                .height(iced::Length::Shrink),
                // expanded state
                expanded_language,
            ))
            // spacer
            .push(iced::widget::horizontal_space())
            // settings button
            .push(
                iced::widget::button(iced::widget::text("SETTINGS").font(font::Font {
                    weight: iced::font::Weight::Bold,
                    ..font::Font::DEFAULT
                }))
                .on_press(Message::OpenSettings)
                .style(Button::Custom(Box::new(RaspirusButtonSecondary)))
                .padding(10),
            )
            // ite allignment
            .align_items(iced::Alignment::Center)
            .spacing(5);

        let title_text = iced::widget::container::Container::new(
            iced::widget::text("RASPIRUS")
                .size(120)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .font(font::Font {
                    weight: iced::font::Weight::Bold,
                    ..font::Font::DEFAULT
                })
                .style(color!(0xd7105e)),
        )
        .padding([0, 0, 10, 0]);

        let mut center_row = iced::widget::Row::new().spacing(5);

        center_row = center_row.push(Space::with_width(iced::Length::FillPortion(2)));

        center_row = match selection {
            LocationSelection::Usb { ref usb } => {
                center_row.push(iced_aw::widgets::DropDown::new(
                    // large button that displays usb and triggers dropdown on click
                    iced::widget::Button::new(
                        iced::widget::Text::new({
                            match usb {
                                Some(usb) => usb.to_string(),
                                None => "No mounted USB devices detected".to_owned(),
                            }
                        })
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .on_press(Message::ToggleUSBSelection)
                    .width(iced::Length::FillPortion(4)),
                    // list of usb devices
                    iced_aw::widgets::SelectionList::new(usbs, |_idx: usize, usb: UsbDevice| {
                        Message::RequestLocation {
                            selection: LocationSelection::Usb { usb: Some(usb) },
                        }
                    })
                    .height(iced::Length::Shrink),
                    expanded_usb,
                ))
            }
            LocationSelection::Folder { ref path } => center_row.push(
                iced::widget::Button::new(
                    iced::widget::Text::new(match path {
                        Some(path) => path.to_string_lossy().to_string(),
                        None => "No folder selected".to_owned(),
                    })
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .width(iced::Length::FillPortion(4))
                .on_press(Message::RequestLocation {
                    selection: LocationSelection::Folder { path: None },
                }),
            ),
            LocationSelection::File { ref path } => center_row.push(
                iced::widget::Button::new(
                    iced::widget::Text::new(match path {
                        Some(path) => path.to_string_lossy().to_string(),
                        None => "No file selected".to_owned(),
                    })
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .width(iced::Length::FillPortion(4))
                .on_press(Message::RequestLocation {
                    selection: LocationSelection::File { path: None },
                }),
            ),
        };

        center_row = center_row.push(iced_aw::widgets::DropDown::new(
            // button to trigger dropdown
            iced::widget::Row::new().push(
                iced::widget::Button::new(
                    iced::widget::Text::new(selection.to_string()).font(iced_aw::BOOTSTRAP_FONT),
                )
                .on_press(Message::ToggleLocationSelection)
                .style(Button::Custom(Box::new(RaspirusButtonOrange))),
            ),
            // dropdown selection list
            iced_aw::widgets::SelectionList::new_with(
                &crate::SELECTION_ICONS,
                |_idx: usize, selection: LocationSelection| Message::LocationChanged { selection },
                16.0,
                3.0,
                iced_aw::SelectionListStyles::default(),
                None,
                iced_aw::BOOTSTRAP_FONT,
            )
            .height(iced::Length::Shrink)
            .width(iced::Length::Shrink)
            .style(SelectionListStyles::custom(RaspirusSelectionList)),
            // expanded state
            expanded_location,
        ));

        center_row = center_row.push(Space::with_width(iced::Length::FillPortion(2)));

        let mut start_button =
            iced::widget::Button::new(iced::widget::text("START").font(font::Font {
                weight: iced::font::Weight::Bold,
                ..font::Font::DEFAULT
            }))
            .style(Button::Custom(Box::new(RaspirusButtonPrimary)))
            .padding([15, 20]);

        match selection {
            LocationSelection::Usb { usb } => {
                if usb.is_some() {
                    start_button = start_button.on_press(Message::StartScan);
                }
            }
            LocationSelection::Folder { path } => {
                if path.is_some() {
                    start_button = start_button.on_press(Message::StartScan)
                }
            }
            LocationSelection::File { path } => {
                if path.is_some() {
                    start_button = start_button.on_press(Message::StartScan)
                }
            }
        }

        let info_button = iced::widget::Button::new(iced::widget::text("INFO").font(font::Font {
            weight: iced::font::Weight::Bold,
            ..font::Font::DEFAULT
        }))
        .style(Button::Custom(Box::new(RaspirusButtonSecondary)))
        .padding([15, 20])
        .on_press(Message::OpenInformation);

        let button_row = iced::widget::Row::new()
            .push(info_button)
            .push(start_button)
            .spacing(10);

        let middle_row = iced::widget::Column::new()
            .push(title_text)
            .push(center_row)
            .push(button_row)
            .align_items(iced::Alignment::Center)
            .spacing(5);

        wrap(
            10,
            iced::widget::Column::new()
                .push(top_row)
                .push(iced::widget::horizontal_rule(5))
                .push(iced::widget::vertical_space())
                .push(middle_row)
                .push(iced::widget::vertical_space())
                .spacing(5)
                .into(),
        )
    }
}
