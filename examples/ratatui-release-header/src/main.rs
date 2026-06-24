use ratasvg::{build_svg_from_widget, svg};
use ratatui::prelude::*;

use crate::header::Header;

mod header;

fn main() -> color_eyre::Result<()> {
    // taken from https://github.com/ratatui/ratatui/blob/main/examples/vhs/release-header.tape
    let opts = ratasvg::Options {
        background_color: "#141432",
        width_px: 1120,
        height_px: 520,
        font_size_px: 25,
    };

    let document = build_svg_from_widget(Header, opts);

    const OUT_FILE: &str = "release-header.svg";

    svg::save(OUT_FILE, &document)?;

    println!("\nsaved to '{}'", OUT_FILE);

    Ok(())
}
