use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};
use svg::{
    Document, Node,
    node::element::{Rectangle, Text},
};

pub use svg;

pub fn build_svg_from_widget(
    widget: impl Widget,
    area: Rect,
    cell_width_px: u16,
    cell_height_px: u16,
) -> Document {
    let mut buf = Buffer::empty(area);

    widget.render(area, &mut buf);
    dbg!(&buf);

    let (document_width_px, document_height_px) =
        (area.width * cell_width_px, area.height * cell_height_px);

    let mut document = Document::new()
        .set("width", document_width_px)
        .set("height", document_height_px)
        .set("viewBox", (0, 0, document_width_px, document_height_px));

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
                        .set("y", (row + 1) * cell_height_px) // +1 for baseline
                        .set("fill", "white")
                        .set("style", "font-family: Monaspace Krypton; font-size: 16px;"),
                );
            }
        }
    }

    document
}
