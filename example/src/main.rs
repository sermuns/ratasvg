use ratasvg::{build_svg_from_widget, svg};
use ratatui::widgets::{RatatuiLogo, RatatuiLogoSize, RatatuiMascot};

use crate::header::Header;

mod header;

fn main() -> color_eyre::Result<()> {
    // https://github.com/ratatui/ratatui/blob/main/examples/vhs/release-header.tape
    svg::save(
        "release-header.svg",
        &build_svg_from_widget(
            Header,
            ratasvg::Options {
                background_color: "#141432",
                width_px: 1120,
                height_px: 520,
                font_size_px: 25,
            },
        ),
    )?;

    svg::save(
        "ratatui-mascot.svg",
        &build_svg_from_widget(
            RatatuiMascot::new(),
            ratasvg::Options {
                background_color: "black",
                width_px: 500,
                height_px: 400,
                font_size_px: 25,
            },
        ),
    )?;

    svg::save(
        "ratatui-logo.svg",
        &build_svg_from_widget(
            RatatuiLogo::small(),
            ratasvg::Options {
                background_color: "black",
                width_px: 840,
                height_px: 150,
                font_size_px: 50,
            },
        ),
    )?;

    Ok(())
}
