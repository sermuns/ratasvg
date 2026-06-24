use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};
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
            if cell.symbol() != " " {
                document.append(
                    Text::new(cell.symbol())
                        .set("x", col * cell_width_px)
                        .set("y", row * cell_height_px),
                );
            }
        }
    }

    document
}
