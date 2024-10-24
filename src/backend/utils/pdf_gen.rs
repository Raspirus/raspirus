use crate::backend::yara_scanner::{Skipped, TaggedFile};
use chrono::NaiveDateTime;
use log::debug;
use rust_i18n::t;
use std::path::PathBuf;

// Try not to change, might break the layout
const PAGE_WIDTH: f32 = 210.0;
const PAGE_HEIGHT: f32 = 297.0;
const FONT_SIZE: f32 = 15.0;
const LINE_HEIGHT: f32 = FONT_SIZE + 6.0;
const LINE_PADDING: f32 = 3.0;
const LINE_PADDING_SECONDARY: f32 = 50.0;
const LOGO_SCALE: f32 = 0.2;
const LOGO_POS_X: f32 = 5.0;
const LOGO_POS_Y: f32 = PAGE_HEIGHT - 17.0;
const TITLE_FONT_SIZE: f32 = 20.0;
const TITLE_POS_X: f32 = 25.0;
const TITLE_POS_Y: f32 = LOGO_POS_Y + 5.0;
const TITLE_SECONDARY_FONT_SIZE: f32 = 15.0;
const TITLE_SECONDARY_POS_X: f32 = 95.0;
const TITLE_SECONDARY_POS_Y: f32 = PAGE_HEIGHT - 40.0;
const PAGE_NUMBER_FONT_SIZE: f32 = FONT_SIZE - 2.0;
const DATE_FONT_SIZE: f32 = 12.0;
const DATE_POS_X: f32 = PAGE_WIDTH - 50.0;
const DATE_POS_Y: f32 = PAGE_HEIGHT - 10.0;
const PAGE_NUMBER_POS_X: f32 = PAGE_WIDTH / 2.0 - 2.0;
const PAGE_NUMBER_POS_Y: f32 = 5.0;
// For the first page only
const FIRST_CONTENT_POS_X: f32 = LOGO_POS_X;
const FIRST_CONTENT_POS_Y: f32 = PAGE_HEIGHT - 55.0;
// For the rest of the pages
const CONTENT_POS_X: f32 = LOGO_POS_X;
const CONTENT_POS_Y: f32 = PAGE_HEIGHT - 10.0;

// NOTE: P(0,0) is bottom left corner
// NOTE: P(x,y) = P(width, height) is top right corner

pub fn generate_pdf(
    skipped: Vec<(Skipped, bool)>,
    tagged: Vec<(TaggedFile, bool)>,
    total: usize,
    log_file: PathBuf,
) -> Result<PathBuf, String> {
    let file_name = log_file.file_name().unwrap_or_default().to_string_lossy();
    let timestamp = file_name.trim_end_matches(".log");

    let downloads_folder = crate::CONFIG
        .lock()
        .map_err(|err| format!("Failed to lock config: {err}"))?
        .paths
        .clone()
        .ok_or_else(|| "No paths?".to_string())?
        .downloads;

    let output_file = downloads_folder.join(format!("{timestamp}.pdf"));

    let (doc, page1, layer1) = printpdf::PdfDocument::new(
        t!("report_title"),
        printpdf::Mm(PAGE_WIDTH),
        printpdf::Mm(PAGE_HEIGHT),
        "Layer 1",
    );
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    // >> START HEADER

    // Load SVG from local file
    let svg = printpdf::Svg::parse(crate::LOGO_VECTOR_STR)
        .map_err(|err| format!("Failed to load logo: {err}"))?;
    // Create a reference to the SVG image (Required for embedding)
    let reference = svg.clone().into_xobject(&current_layer);
    // Embed the SVG image on the page
    reference.clone().add_to_layer(
        &current_layer,
        printpdf::SvgTransform {
            translate_x: Some(printpdf::Mm(LOGO_POS_X).into_pt()),
            translate_y: Some(printpdf::Mm(LOGO_POS_Y).into_pt()),
            scale_x: Some(LOGO_SCALE),
            scale_y: Some(LOGO_SCALE),
            ..Default::default()
        },
    );
    // Use a built-in font
    let font = doc
        .add_builtin_font(printpdf::BuiltinFont::Helvetica)
        .map_err(|err| format!("Failed to add fonts: {err}"))?;

    // Add a title to the page
    current_layer.use_text(
        t!("report_title"),
        TITLE_FONT_SIZE,
        printpdf::Mm(TITLE_POS_X),
        printpdf::Mm(TITLE_POS_Y),
        &font,
    );

    let date_string = NaiveDateTime::parse_from_str(timestamp, "%Y_%m_%d_%H_%M_%S")
        .map_err(|err| format!("Failed to parse timestamp to time: {err}"))?;
    let date_string = date_string.format("%Y/%m/%d %H:%M:%S").to_string();

    // Add the date to the page
    current_layer.use_text(
        date_string,
        DATE_FONT_SIZE,
        printpdf::Mm(DATE_POS_X),
        printpdf::Mm(DATE_POS_Y),
        &font,
    );

    // Add a line below the title (Set the start and end points)
    let line_start_point = printpdf::Point::new(
        printpdf::Mm(LOGO_POS_X - 2.0),
        printpdf::Mm(LOGO_POS_Y - LINE_PADDING),
    );

    let line_end_point = printpdf::Point::new(
        printpdf::Mm(PAGE_WIDTH - LINE_PADDING),
        printpdf::Mm(LOGO_POS_Y - LINE_PADDING),
    );

    // Line to separate the title from the content
    current_layer.add_line(printpdf::Line {
        points: vec![(line_start_point, true), (line_end_point, true)],
        is_closed: true,
    });

    // Add detection summary
    current_layer.use_text(
        format!(
            "{}: {} | {}: {} | {}: {}",
            t!("report_malware"),
            tagged.len(),
            t!("report_skipped"),
            skipped.len(),
            t!("report_scanned"),
            total
        ),
        12.0,
        printpdf::Mm(LOGO_POS_X),
        printpdf::Mm(LOGO_POS_Y - 10.0),
        &font,
    );

    // << HEADER IS DONE

    // Add content (VERY SIMILAR TO OLD CODE)
    // weird code with magic numbers but what can you do /shrug
    let pt_in_mm = (LINE_HEIGHT) * 0.3537778;
    let max_lines_page = (FIRST_CONTENT_POS_Y / pt_in_mm) as usize;
    let max_chars_per_line = (PAGE_WIDTH - 10.0) / (FONT_SIZE * 0.5 * 0.3537778);
    debug!("Determined lines per page should be {max_lines_page}, and max chars per line should be {max_chars_per_line}");

    // If both vectors are empty, we simply add a message to the first page
    if tagged.is_empty() && skipped.is_empty() {
        current_layer.begin_text_section();
        current_layer.set_font(&font, FONT_SIZE);
        current_layer.set_text_cursor(
            printpdf::Mm(FIRST_CONTENT_POS_X),
            printpdf::Mm(FIRST_CONTENT_POS_Y),
        );
        current_layer.set_line_height(LINE_HEIGHT);
        current_layer.write_text(t!("report_no_malware"), &font);
        current_layer.add_line_break();
        current_layer.write_text(t!("report_no_skipped"), &font);
        current_layer.end_text_section();
    }

    // Add a secondary title if one of the two arrays is empty
    if !tagged.is_empty() {
        current_layer.use_text(
            t!("report_malware"),
            TITLE_SECONDARY_FONT_SIZE,
            printpdf::Mm(TITLE_SECONDARY_POS_X),
            printpdf::Mm(TITLE_SECONDARY_POS_Y),
            &font,
        );
    } else if !skipped.is_empty() {
        current_layer.use_text(
            t!("report_skiped"),
            TITLE_SECONDARY_FONT_SIZE,
            printpdf::Mm(TITLE_SECONDARY_POS_X),
            printpdf::Mm(TITLE_SECONDARY_POS_Y),
            &font,
        );
    }

    if !tagged.is_empty() || !skipped.is_empty() {
        // Add a line below the title (Set the start and end points)
        let line_start_point = printpdf::Point::new(
            printpdf::Mm(LINE_PADDING_SECONDARY),
            printpdf::Mm(TITLE_SECONDARY_POS_Y - LINE_PADDING),
        );
        let line_end_point = printpdf::Point::new(
            printpdf::Mm(PAGE_WIDTH - LINE_PADDING_SECONDARY),
            printpdf::Mm(TITLE_SECONDARY_POS_Y - LINE_PADDING),
        );
        // Line to separate the title from the content
        current_layer.add_line(printpdf::Line {
            points: vec![(line_start_point, true), (line_end_point, true)],
            is_closed: true,
        });
    }

    // start with the first page
    current_layer.begin_text_section();
    current_layer.set_font(&font, FONT_SIZE);
    current_layer.set_text_cursor(
        printpdf::Mm(FIRST_CONTENT_POS_X),
        printpdf::Mm(FIRST_CONTENT_POS_Y),
    );
    current_layer.set_line_height(LINE_HEIGHT);

    // Counter for the current line and page
    let mut current_line = 1;
    let mut current_page = 1;

    let mut tagged_lines = Vec::new();
    // add tagged files to the total lines
    for tagged_file in &tagged {
        // add initial 'header' for a particular tagged file
        tagged_lines.push(format!(
            "• [{}] {}",
            tagged_file.0.rule_count,
            tagged_file.0.path.to_string_lossy()
        ));
        tagged_lines.extend(
            tagged_file
                .0
                .descriptions
                .iter()
                .map(|description| format!("\t {}", description)),
        );
    }

    let mut skipped_lines = Vec::new();
    // add skipped files to the total lines
    for skipped_file in &skipped {
        skipped_lines.push(format!(
            "• {} {}",
            skipped_file.0.path.to_string_lossy(),
            skipped_file.0.reason
        ));
    }

    for line in tagged_lines {
        // if we reach the maximum lines per page we create a new one
        if current_line % max_lines_page == 0 && current_line > 0 {
            current_page += 1;
            // cleanup old page
            current_layer.end_text_section();

            debug!("Page end reached, creating new page at line {current_line}");
            let (page, layer) = doc.add_page(
                printpdf::Mm(PAGE_WIDTH),
                printpdf::Mm(PAGE_HEIGHT),
                "Layer 1",
            );

            // prepare new page
            current_layer = doc.get_page(page).get_layer(layer);
            current_layer.begin_text_section();
            current_layer.set_font(&font, FONT_SIZE);

            current_layer.set_text_cursor(printpdf::Mm(CONTENT_POS_X), printpdf::Mm(CONTENT_POS_Y));
            current_layer.set_line_height(LINE_HEIGHT);
        }

        // write the current line to pdf
        let mut lines = Vec::new();

        // split line into pieces that fit on the page
        lines.extend(
            line.chars()
                .collect::<Vec<char>>()
                .chunks(max_chars_per_line as usize)
                .map(|chars| chars.iter().collect())
                .collect::<Vec<String>>(),
        );

        for line in lines {
            current_layer.write_text(line, &font);
            current_layer.add_line_break();
            current_line += 1;
        }
    }

    // Reset the current line and page for the skipped files
    current_line = 1;

    // If both are not empty, we create a new page for the skipped files
    if !skipped.is_empty() && !tagged.is_empty() {
        let (page, layer) = doc.add_page(
            printpdf::Mm(PAGE_WIDTH),
            printpdf::Mm(PAGE_HEIGHT),
            "Layer 1",
        );
        // prepare new page
        current_layer = doc.get_page(page).get_layer(layer);
        current_layer.use_text(
            t!("report_skipped"),
            TITLE_SECONDARY_FONT_SIZE,
            printpdf::Mm(TITLE_SECONDARY_POS_X),
            printpdf::Mm(CONTENT_POS_Y),
            &font,
        );
        // Add a line below the title (Set the start and end points)
        let line_start_point = printpdf::Point::new(
            printpdf::Mm(LINE_PADDING_SECONDARY),
            printpdf::Mm(CONTENT_POS_Y - LINE_PADDING),
        );
        let line_end_point = printpdf::Point::new(
            printpdf::Mm(PAGE_WIDTH - LINE_PADDING_SECONDARY),
            printpdf::Mm(CONTENT_POS_Y - LINE_PADDING),
        );
        // Line to separate the title from the content
        current_layer.add_line(printpdf::Line {
            points: vec![(line_start_point, true), (line_end_point, true)],
            is_closed: true,
        });
        current_layer.begin_text_section();
        current_layer.set_font(&font, FONT_SIZE);

        current_layer.set_text_cursor(
            printpdf::Mm(CONTENT_POS_X),
            printpdf::Mm(CONTENT_POS_Y - 15.0),
        );
        current_layer.set_line_height(LINE_HEIGHT);
        current_page += 1;
        current_line = 1; // Reset current line for the new page
    } else {
        // start with the first page
        current_layer.begin_text_section();
        current_layer.set_font(&font, FONT_SIZE);
        current_layer.set_text_cursor(
            printpdf::Mm(FIRST_CONTENT_POS_X),
            printpdf::Mm(FIRST_CONTENT_POS_Y),
        );
        current_layer.set_line_height(LINE_HEIGHT);
    }

    for line in skipped_lines {
        // if we reach the maximum lines per page we create a new one
        if current_line % max_lines_page == 0 && current_line > 0 {
            current_page += 1;
            // cleanup old page
            current_layer.end_text_section();

            debug!("Page end reached, creating new page at line {current_line}");
            let (page, layer) = doc.add_page(
                printpdf::Mm(PAGE_WIDTH),
                printpdf::Mm(PAGE_HEIGHT),
                "Layer 1",
            );

            // prepare new page
            current_layer = doc.get_page(page).get_layer(layer);
            current_layer.begin_text_section();
            current_layer.set_font(&font, FONT_SIZE);

            current_layer.set_text_cursor(printpdf::Mm(CONTENT_POS_X), printpdf::Mm(CONTENT_POS_Y));
            current_layer.set_line_height(LINE_HEIGHT);
        }

        // write the current line to pdf
        let mut lines = Vec::new();

        // split line into pieces that fit on the page
        lines.extend(
            line.chars()
                .collect::<Vec<char>>()
                .chunks(max_chars_per_line as usize)
                .map(|chars| chars.iter().collect())
                .collect::<Vec<String>>(),
        );

        for line in lines {
            current_layer.write_text(line, &font);
            current_layer.add_line_break();
            current_line += 1;
        }
    }
    current_layer.end_text_section();
    // Finally write the page number to the page
    current_layer.use_text(
        current_page.to_string(),
        PAGE_NUMBER_FONT_SIZE,
        printpdf::Mm(PAGE_NUMBER_POS_X),
        printpdf::Mm(PAGE_NUMBER_POS_Y),
        &font,
    );

    // Save the PDF to a file
    let pdf_bytes = doc
        .save_to_bytes()
        .map_err(|err| format!("Failed to conver pdf to bytes: {err}"))?;
    std::fs::write(&output_file, &pdf_bytes)
        .map_err(|err| format!("Failed to save pdf to file: {err}"))?;

    Ok(output_file)
}
