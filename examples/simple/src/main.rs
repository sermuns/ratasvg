use ratasvg::{build_svg_from_widget, svg};
use ratatui::prelude::*;

struct Nothing;

impl Widget for &Nothing {
    fn render(self, area: Rect, buf: &mut Buffer) {
        "hehe".render(area, buf);
    }
}

fn main() -> color_eyre::Result<()> {
    let nothing = Nothing;
    let area = Rect {
        width: 20,
        height: 10,
        ..Default::default()
    };

    let document = build_svg_from_widget(&nothing, area, 10, 18);

    const OUT_FILE: &str = "out.svg";

    svg::save(OUT_FILE, &document)?;

    println!("\nsaved to '{}'", OUT_FILE);

    Ok(())
}
