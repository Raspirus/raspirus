use iced::{font, widget::Space};
use rust_i18n::t;

use crate::{
    backend::utils::usb_utils::UsbDevice,
    frontend::{
        iced::{wrap, LocationSelection, Message, Raspirus},
        theme::{
            button::{
                button_orange_style, button_primary_style, button_secondary_style,
                button_select_style, button_transparent_style,
            },
            selection_list::selection_list_style,
            PRIMARY_COLOR,
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
                    iced::widget::Button::new(iced::widget::Text::new(
                        rust_i18n::locale().to_string(),
                    ))
                    .on_press(Message::ToggleLanguageSelection),
                ),
                // dropdown selection list
                iced_aw::widgets::SelectionList::new(
                    &crate::SUPPORTED_LANGUAGES,
                    |_idx: usize, language: String| {
                        rust_i18n::set_locale(&language);
                        Message::LanguageChanged { language }
                    },
                )
                .height(iced::Length::Shrink),
                // expanded state
                expanded_language,
            ))
            // spacer
            .push(iced::widget::horizontal_space())
            // settings button
            .push(
                iced::widget::button(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::text(iced_fonts::Bootstrap::GearFill.to_string())
                                .font(iced_fonts::BOOTSTRAP_FONT),
                        )
                        .push(iced::widget::text(t!("settings")).font(font::Font {
                            weight: iced::font::Weight::Bold,
                            ..font::Font::DEFAULT
                        })),
                )
                .on_press(Message::OpenSettings)
                .style(button_secondary_style)
                .padding(10),
            )
            .align_y(iced::Alignment::Center)
            .spacing(5);

        let title_text = iced::widget::container::Container::new(
            iced::widget::text("RASPIRUS")
                .size(120)
                .align_x(iced::alignment::Horizontal::Center)
                .font(font::Font {
                    weight: iced::font::Weight::Bold,
                    ..font::Font::DEFAULT
                })
                .style(|_| iced::widget::text::Style {
                    color: Some(PRIMARY_COLOR),
                }),
        );

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
                                None => t!("usb_list_not_found").to_string(),
                            }
                        })
                        .align_x(iced::alignment::Horizontal::Center),
                    )
                    .on_press(Message::ToggleUSBSelection)
                    .width(iced::Length::FillPortion(4))
                    .style(button_select_style),
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
                        None => t!("folder_selection_not").to_owned().to_string(),
                    })
                    .align_x(iced::alignment::Horizontal::Center),
                )
                .width(iced::Length::FillPortion(4))
                .on_press(Message::RequestLocation {
                    selection: LocationSelection::Folder { path: None },
                })
                .style(button_select_style),
            ),
            LocationSelection::File { ref path } => center_row.push(
                iced::widget::Button::new(
                    iced::widget::Text::new(match path {
                        Some(path) => path.to_string_lossy().to_string(),
                        None => t!("file_selection_not").to_string(),
                    })
                    .align_x(iced::alignment::Horizontal::Center),
                )
                .width(iced::Length::FillPortion(4))
                .on_press(Message::RequestLocation {
                    selection: LocationSelection::File { path: None },
                })
                .style(button_select_style),
            ),
        };

        center_row = center_row.push(iced_aw::widgets::DropDown::new(
            // button to trigger dropdown
            iced::widget::Row::new().push(
                iced::widget::Button::new(
                    iced::widget::Text::new(selection.to_string()).font(iced_fonts::BOOTSTRAP_FONT),
                )
                .on_press(Message::ToggleLocationSelection)
                .style(button_orange_style),
            ),
            // dropdown selection list
            iced_aw::widget::SelectionList::new_with(
                &crate::SELECTION_ICONS,
                |_idx: usize, selection: LocationSelection| Message::LocationChanged { selection },
                16.0,
                5.0,
                selection_list_style,
                None,
                iced_fonts::BOOTSTRAP_FONT,
            )
            .height(iced::Length::Shrink)
            .width(iced::Length::Shrink),
            // expanded state
            expanded_location,
        ));

        center_row = center_row.push(Space::with_width(iced::Length::FillPortion(2)));

        let mut start_button = iced::widget::Button::new(
            iced::widget::text(t!("start").to_uppercase()).font(font::Font {
                weight: iced::font::Weight::Bold,
                ..font::Font::DEFAULT
            }),
        )
        .style(button_primary_style)
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

        let info_button = iced::widget::Button::new(
            iced::widget::text(t!("info").to_uppercase()).font(font::Font {
                weight: iced::font::Weight::Bold,
                ..font::Font::DEFAULT
            }),
        )
        .style(button_secondary_style)
        .padding([15, 20])
        .on_press(Message::OpenInformation);

        let button_row = iced::widget::Row::new()
            .push(info_button)
            .push(start_button)
            .spacing(10)
            .padding([10, 0]);

        let bottom_text = iced::widget::container(
            iced::widget::Row::new()
                .push(
                    iced::widget::text(t!("terms_part_1"))
                        .align_x(iced::alignment::Horizontal::Center),
                )
                .push(
                    iced::widget::button(iced::widget::text(t!("terms_part_2")))
                        .style(button_transparent_style)
                        .on_press(Message::OpenTerms),
                )
                .align_y(iced::Alignment::Center),
        )
        .padding([10, 0]);

        let middle_row = iced::widget::Column::new()
            .push(title_text)
            .push(center_row)
            .push(button_row)
            .push(bottom_text)
            .align_x(iced::Alignment::Center)
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
