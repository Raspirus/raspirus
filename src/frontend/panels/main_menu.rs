use iced::{font, widget::Space};
use rust_i18n::t;

use crate::{
    backend::utils::usb_utils::UsbDevice,
    frontend::{
        iced::{wrap, Language, LocationSelection, Message, Raspirus},
        theme::{
            button::{
                button_blue_style, button_orange_style, button_primary_style, button_secondary_style, button_select_style, button_selectionlist, button_selectionlist_lang, button_selectionlist_selected_lang, button_transparent_style
            },
            selection_list::{selectionlist, selectionlist_lang},
            svg::svg_icon,
            PRIMARY_COLOR, SECONDARY_COLOR,
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
        let current_language = match crate::SUPPORTED_LANGUAGES
            .iter()
            .find(|language| language.file_name.eq(&rust_i18n::locale().to_string()))
        {
            Some(language) => language,
            None => &Language::new("not_found", "not_found", crate::EXCLAMATION_CIRCLE),
        };
        let mut language_list = Vec::new();
        for language in crate::SUPPORTED_LANGUAGES.iter() {
            language_list.push(
                iced::widget::Button::new(
                    iced::widget::Row::new()
                        .push(
                            iced::widget::svg(language.flag.clone())
                                .height(20)
                                .width(iced::Length::Shrink),
                        )
                        .push(
                            iced::widget::Text::new(&language.display_name)
                                .wrapping(iced::widget::text::Wrapping::None),
                        )
                        .spacing(10),
                )
                .on_press(Message::LanguageChanged {
                    language: language.file_name.clone(),
                })
                .style(if language.file_name.eq(&current_language.file_name) {
                    button_selectionlist_selected_lang
                } else {
                    button_selectionlist_lang
                })
                .width(iced::Length::Fill)
                .into(),
            )
        }
        let language_selection = selectionlist_lang(language_list);

        let top_row = iced::widget::Row::new()
            // language selection
            .push(
                iced_aw::widgets::DropDown::new(
                    // button to trigger dropdown
                    iced::widget::Button::new(
                        iced::widget::Row::new()
                            .push(
                                iced::widget::svg(current_language.flag.clone())
                                    .height(20)
                                    .width(iced::Length::Shrink),
                            )
                            .push(
                                iced::widget::Text::new(current_language.display_name.clone())
                                    .wrapping(iced::widget::text::Wrapping::None),
                            )
                            .spacing(10)
                            .padding([0, 5]),
                    )
                    .padding(10)
                    .style(button_blue_style)
                    .on_press(Message::ToggleLanguageSelection),
                    // dropdown selection list
                    language_selection,
                    // expanded state
                    expanded_language,
                )
                //.width(125)
                .on_dismiss(Message::ToggleLanguageSelection),
            )
            // spacer
            .push(iced::widget::horizontal_space())
            // settings button
            .push(
                iced::widget::button::Button::new(
                    iced::widget::Row::new()
                        .push(
                            svg_icon(crate::SETTINGS).style(|_, _1| iced::widget::svg::Style {
                                color: Some(SECONDARY_COLOR),
                            }),
                        )
                        .push(iced::widget::text(t!("settings")).font(font::Font {
                            weight: iced::font::Weight::Bold,
                            ..font::Font::DEFAULT
                        }))
                        .spacing(10)
                        .align_y(iced::Alignment::Center),
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
                    .padding(7)
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
                .padding(7)
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
                .padding(7)
                .style(button_select_style),
            ),
        };

        let options = crate::TARGET_SELECTIONS
            .iter()
            .map(|element| {
                iced::widget::Button::new(
                    iced::widget::svg(iced::widget::svg::Handle::from_memory(element.1))
                        .width(iced::Length::Shrink)
                        .opacity(if element.0.eq(&selection) { 1.0 } else { 0.75 }),
                )
                .on_press(Message::LocationChanged {
                    selection: element.0.clone(),
                })
                .style(button_selectionlist)
                .into()
            })
            .collect();

        let type_selection = selectionlist(options);
        center_row = center_row.push(
            iced_aw::widgets::DropDown::new(
                // button to trigger dropdown
                iced::widget::Row::new().push(
                    iced::widget::Button::new(
                        iced::widget::svg(iced::widget::svg::Handle::from_memory(
                            match selection {
                                LocationSelection::Usb { .. } => crate::USB,
                                LocationSelection::Folder { .. } => crate::FOLDER,
                                LocationSelection::File { .. } => crate::FILE,
                            },
                        ))
                        .width(iced::Length::Shrink),
                    )
                    .on_press(Message::ToggleLocationSelection)
                    .style(button_orange_style),
                ),
                // dropdown selection list
                type_selection,
                // expanded state
                expanded_location,
            )
            .on_dismiss(if expanded_location {
                Message::ToggleLocationSelection
            } else {
                Message::None
            }),
        );

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
