use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};
use svg::{
    Document, Node,
    node::element::{Rectangle, Text},
};

pub use svg;

pub struct Options {
    pub area: Rect,
    pub cell_width_px: u16,
    pub cell_height_px: u16,
    pub font_size_px: u16,
}

pub fn build_svg_from_widget(widget: impl Widget, options: Options) -> Document {
    let Options {
        area,
        cell_width_px,
        cell_height_px,
        font_size_px,
    } = options;

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
                "font-family: Monaspace Krypton;",
                &format!("font-size: {}px;", font_size_px),
                "fill: white",
            ]
            .concat(),
        )
        .set("dominant-baseline", "central");

    let background_rect = Rectangle::new()
        .set("fill", "#333")
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
                        .set("y", row * cell_height_px), // +1 for baseline
                );
            }
        }
    }

    document
}
