// Try not to change, might break the layout
const PAGE_WIDTH: f32 = 210.0;
const PAGE_HEIGHT: f32 = 297.0;
const FONT_SIZE: f32 = 15.0;
const LINE_HEIGHT: f32 = FONT_SIZE + 2.0;
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
const DATE_POS_X: f32 = PAGE_WIDTH - 28.0;
const DATE_POS_Y: f32 = PAGE_HEIGHT - 7.0;
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

// Example struct to hold the data. File path and yara rule description
#[derive(Debug, Clone)]
struct Detection {
    file_path: String,
    yara_rule: String,
}

fn main() {
    // TODO: TO CHANGE
    const TITLE: &str = "Raspirus Detection Report";
    const TITLE_MALWARE: &str = "Malware";
    const TITLE_SKIPPED: &str = "Skipped";
    const SVG: &str = include_str!("../assets/logo.svg");
    const DATE: &str = "2021-09-01";
    const DETECTION_SUMMARY: &str = "Found 0 malware | Skipped 10 files | Scanned 1274 files";
    const NO_DETECTIONS: &str = "No malware detected";
    const NO_SKIPPED: &str = "No files skipped";
    const PDF_PATH: &str = "test_svg.pdf";
    // TODO: Remove this test content
    let malware: Vec<Detection> = vec![];
    let skipped: Vec<Detection> = vec![
    ];

    let (doc, page1, layer1) = printpdf::PdfDocument::new(TITLE, printpdf::Mm(PAGE_WIDTH), printpdf::Mm(PAGE_HEIGHT), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    // >> START HEADER

    // Load SVG from local file
    let svg = printpdf::Svg::parse(SVG).unwrap();
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
    let font = doc.add_builtin_font(printpdf::BuiltinFont::Helvetica).unwrap();
    // Add a title to the page
    current_layer.use_text(
        TITLE,
        TITLE_FONT_SIZE,
        printpdf::Mm(TITLE_POS_X),
        printpdf::Mm(TITLE_POS_Y),
        &font,
    );
    // Add the date to the page
    current_layer.use_text(DATE, DATE_FONT_SIZE, printpdf::Mm(DATE_POS_X), printpdf::Mm(DATE_POS_Y), &font);

    // Add a line below the title (Set the start and end points)
    let line_start_point = printpdf::Point::new(printpdf::Mm(LOGO_POS_X - 2.0), printpdf::Mm(LOGO_POS_Y - LINE_PADDING));
    let line_end_point = printpdf::Point::new(printpdf::Mm(PAGE_WIDTH - LINE_PADDING), printpdf::Mm(LOGO_POS_Y - LINE_PADDING));
    // Line to separate the title from the content
    current_layer.add_line(printpdf::Line {
        points: vec![(line_start_point, true), (line_end_point, true)],
        is_closed: true,
    });

    // Add detection summary
    current_layer.use_text(
        DETECTION_SUMMARY,
        12.0,
        printpdf::Mm(LOGO_POS_X),
        printpdf::Mm(LOGO_POS_Y - 10.0),
        &font,
    );

    // << HEADER IS DONE

    // Add content (VERY SIMILAR TO OLD CODE)
    // weird code with magic numbers but what can you do /shrug
    let pt_in_mm = (LINE_HEIGHT) * 0.3537778;
    let mut max_lines_page = (FIRST_CONTENT_POS_Y / pt_in_mm) as usize;
    let max_chars_per_line = (PAGE_WIDTH - 10.0) / (FONT_SIZE * 0.5 * 0.3537778);
    println!("Determined lines per page should be {max_lines_page}, and max chars per line should be {max_chars_per_line}");

    // If both vectors are empty, we simply add a message to the first page
    if malware.is_empty() && skipped.is_empty() {
        current_layer.begin_text_section();
        current_layer.set_font(&font, FONT_SIZE);
        current_layer.set_text_cursor(printpdf::Mm(FIRST_CONTENT_POS_X), printpdf::Mm(FIRST_CONTENT_POS_Y));
        current_layer.set_line_height(LINE_HEIGHT);
        current_layer.write_text(NO_DETECTIONS, &font);
        current_layer.add_line_break();
        current_layer.write_text(NO_SKIPPED, &font);
        current_layer.end_text_section();
    }

    // Add a secondary title if one of the two arrays is empty
    if !malware.is_empty() {
        current_layer.use_text(TITLE_MALWARE, 
        TITLE_SECONDARY_FONT_SIZE, printpdf::Mm(TITLE_SECONDARY_POS_X), printpdf::Mm(TITLE_SECONDARY_POS_Y), &font);
    } else if !skipped.is_empty() {
        current_layer.use_text(TITLE_SKIPPED, 
        TITLE_SECONDARY_FONT_SIZE, printpdf::Mm(TITLE_SECONDARY_POS_X), printpdf::Mm(TITLE_SECONDARY_POS_Y), &font);
    }

    if !malware.is_empty() || !skipped.is_empty() {
        // Add a line below the title (Set the start and end points)
        let line_start_point = printpdf::Point::new(printpdf::Mm(LINE_PADDING_SECONDARY), printpdf::Mm(TITLE_SECONDARY_POS_Y - LINE_PADDING));
        let line_end_point = printpdf::Point::new(printpdf::Mm(PAGE_WIDTH - LINE_PADDING_SECONDARY), printpdf::Mm(TITLE_SECONDARY_POS_Y - LINE_PADDING));
        // Line to separate the title from the content
        current_layer.add_line(printpdf::Line {
            points: vec![(line_start_point, true), (line_end_point, true)],
            is_closed: true,
        });
    }

    // start with the first page
    current_layer.begin_text_section();
    current_layer.set_font(&font, FONT_SIZE);
    current_layer.set_text_cursor(printpdf::Mm(FIRST_CONTENT_POS_X), printpdf::Mm(FIRST_CONTENT_POS_Y));
    current_layer.set_line_height(LINE_HEIGHT);

    // Counter for the current line and page
    let mut current_line = 1;
    let mut current_page = 1;

    // TODO: Replace malware with std::io::BufReader::new(log).lines()
    for line in malware.clone() {
        // if we reach the maximum lines per page we create a new one
        if current_line % max_lines_page == 0 && current_line > 0 {
            println!("Reached max lines per page, creating new page");
            println!("Current line is {current_line}");
            println!("Current max lines per page is {max_lines_page}");

            // cleanup old page
            current_layer.end_text_section();
            // Write the page number to the previous page
            current_layer.use_text(
                current_page.to_string(),
                PAGE_NUMBER_FONT_SIZE,
                printpdf::Mm(PAGE_NUMBER_POS_X),
                printpdf::Mm(PAGE_NUMBER_POS_Y),
                &font,
            );

            let (page, layer) = doc.add_page(
                printpdf::Mm(PAGE_WIDTH),
                printpdf::Mm(PAGE_HEIGHT),
                "Layer 1",
            );

            // Set the new max lines per page as its different from the first page
            max_lines_page = (CONTENT_POS_Y / pt_in_mm) as usize;

            // prepare new page
            current_layer = doc.get_page(page).get_layer(layer);
            current_layer.begin_text_section();
            current_layer.set_font(&font, FONT_SIZE);

            current_layer.set_text_cursor(printpdf::Mm(CONTENT_POS_X), printpdf::Mm(CONTENT_POS_Y));
            current_layer.set_line_height(LINE_HEIGHT);
            current_page += 1;
            current_line = 1; // Reset current line for the new page
        }

        // write the current line to pdf
        // TODO: Replace with match statement from old code!
        let mut lines = Vec::new();

        // concatenate file_path and yara_rule with " - "
        let combined_line = format!("• {} - {}", line.file_path, line.yara_rule);

        // split combined_line into pieces that fit on the page
        lines.extend(
            combined_line
                .chars()
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

    // Reset the current line and page for the skipped files
    current_line = 1;

    // If both are not empty, we create a new page for the skipped files
    if !skipped.is_empty() && !malware.is_empty() {
        let (page, layer) = doc.add_page(
            printpdf::Mm(PAGE_WIDTH),
            printpdf::Mm(PAGE_HEIGHT),
            "Layer 1",
        );
            // prepare new page
            current_layer = doc.get_page(page).get_layer(layer);
            current_layer.use_text(TITLE_SKIPPED, 
                TITLE_SECONDARY_FONT_SIZE, printpdf::Mm(TITLE_SECONDARY_POS_X), printpdf::Mm(PAGE_HEIGHT - 10.0), &font);
            // Add a line below the title (Set the start and end points)
            let line_start_point = printpdf::Point::new(printpdf::Mm(LINE_PADDING_SECONDARY), printpdf::Mm(PAGE_HEIGHT - 10.0 - LINE_PADDING));
            let line_end_point = printpdf::Point::new(printpdf::Mm(PAGE_WIDTH - LINE_PADDING_SECONDARY), printpdf::Mm(PAGE_HEIGHT - 10.0 - LINE_PADDING));
            // Line to separate the title from the content
            current_layer.add_line(printpdf::Line {
                points: vec![(line_start_point, true), (line_end_point, true)],
                is_closed: true,
            });
            current_layer.begin_text_section();
            current_layer.set_font(&font, FONT_SIZE);

            current_layer.set_text_cursor(printpdf::Mm(CONTENT_POS_X), printpdf::Mm(CONTENT_POS_Y - 15.0));
            current_layer.set_line_height(LINE_HEIGHT);
            current_page += 1;
            current_line = 1; // Reset current line for the new page
    } else {
        // start with the first page
        current_layer.begin_text_section();
        current_layer.set_font(&font, FONT_SIZE);
        current_layer.set_text_cursor(printpdf::Mm(FIRST_CONTENT_POS_X), printpdf::Mm(FIRST_CONTENT_POS_Y));
        current_layer.set_line_height(LINE_HEIGHT);
    }

    for line in skipped {
        // if we reach the maximum lines per page we create a new one
        if current_line % max_lines_page == 0 && current_line > 0 {
            println!("Reached max lines per page, creating new page");
            println!("Current line is {current_line}");
            println!("Current max lines per page is {max_lines_page}");

            // cleanup old page
            current_layer.end_text_section();
            // Write page number to the last page
            current_layer.use_text(
                current_page.to_string(),
                PAGE_NUMBER_FONT_SIZE,
                printpdf::Mm(PAGE_NUMBER_POS_X),
                printpdf::Mm(PAGE_NUMBER_POS_Y),
                &font,
            );

            let (page, layer) = doc.add_page(
                printpdf::Mm(PAGE_WIDTH),
                printpdf::Mm(PAGE_HEIGHT),
                "Layer 1",
            );

            // Set the new max lines per page
            max_lines_page = (CONTENT_POS_Y / pt_in_mm) as usize;

            // prepare new page
            current_layer = doc.get_page(page).get_layer(layer);
            current_layer.begin_text_section();
            current_layer.set_font(&font, FONT_SIZE);

            current_layer.set_text_cursor(printpdf::Mm(CONTENT_POS_X), printpdf::Mm(CONTENT_POS_Y));
            current_layer.set_line_height(LINE_HEIGHT);
            current_page += 1;
            current_line = 1; // Reset current line for the new page
        }

        // write the current line to pdf
        // TODO: Replace with match statement from old code!
        let mut lines = Vec::new();

        // concatenate file_path and yara_rule with " - "
        let combined_line = format!("• {} - {}", line.file_path, line.yara_rule);

        // split combined_line into pieces that fit on the page
        lines.extend(
            combined_line
                .chars()
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
    let pdf_bytes = doc.save_to_bytes().unwrap();
    std::fs::write(PDF_PATH, &pdf_bytes).unwrap();
}
