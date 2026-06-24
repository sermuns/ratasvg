use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::Widget,
};
use svg::{
    Document, Node,
    node::element::{Rectangle, Text},
};

pub use svg;

pub struct Options<'a> {
    pub background_color: &'a str,
    pub width_px: u16,
    pub height_px: u16,
    pub font_size_px: u16,
}

pub fn build_svg_from_widget(
    widget: impl Widget,
    Options {
        background_color,
        width_px,
        height_px,
        font_size_px,
    }: Options,
) -> Document {
    // FIXME: do it properly..
    const FONT_ASPECT_RATIO: f32 = 3. / 5.;
    let cell_width_px = (font_size_px as f32 * FONT_ASPECT_RATIO) as u16;
    let cell_height_px = font_size_px;

    let area = Rect {
        width: width_px / cell_width_px,
        height: height_px / cell_height_px,
        ..Default::default()
    };
    let mut buf = Buffer::empty(area);

    widget.render(area, &mut buf);
    dbg!(&buf);

    let (document_width_px, document_height_px) =
        (area.width * cell_width_px, area.height * cell_height_px);

    let mut document = Document::new()
        .set("width", document_width_px)
        .set("height", document_height_px)
        .set("viewBox", (0, 0, document_width_px, document_height_px))
        .set(
            "style",
            [
                "font-family:DejaVu Sans Mono;",
                &format!("font-size: {}px;", font_size_px),
                "fill: white",
            ]
            .concat(),
        )
        .set("dominant-baseline", "central");

    let background_rect = Rectangle::new()
        .set("fill", background_color)
        .set("width", document_width_px)
        .set("height", document_height_px);
    document.append(background_rect);

    for col in 0..area.width {
        for row in 0..area.height {
            let cell = &buf[(col, row)];
            match cell.symbol() {
                " " => continue,
                symbol => {
                    let text = Text::new(symbol)
                        .set("x", col * cell_width_px)
                        .set("y", (row + 1) * cell_height_px) // account for baseline??
                        .set("style", ratatui_style_to_css_style_string(cell.style()));

                    document.append(text);
                }
            }
        }
    }

    document
}

fn ratatui_style_to_css_style_string(style: Style) -> String {
    let mut css_style = String::new();

    if let Some(fg) = style.fg {
        css_style.push_str(&format!("fill:{};", fg));
    }

    // if let Some(bg) = style.bg {
    //     css_style.push_str(&format!("background-color:{};", bg));
    // }

    if style.add_modifier.contains(Modifier::ITALIC) {
        css_style.push_str("font-style:italic;");
    }
    if style.add_modifier.contains(Modifier::DIM) {
        css_style.push_str("opacity:0.5;");
    }

    css_style
}
