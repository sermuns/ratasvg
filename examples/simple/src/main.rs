use ratasvg::{build_svg_from_widget, svg};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

struct Something;

impl Widget for Something {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("hehe")
            .block(Block::bordered())
            .render(area, buf);
    }
}

fn main() -> color_eyre::Result<()> {
    let something = Something;

    let area = Rect {
        width: 20,
        height: 10,
        ..Default::default()
    };

    let opts = ratasvg::Options {
        area,
        cell_width_px: 9,
        cell_height_px: 18,
        font_size_px: 14,
    };
    let document = build_svg_from_widget(something, opts);

    const OUT_FILE: &str = "out.svg";

    svg::save(OUT_FILE, &document)?;

    println!("\nsaved to '{}'", OUT_FILE);

    Ok(())
}
