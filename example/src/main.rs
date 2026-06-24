use std::error::Error;

use mousefood_extras::MouseFoodLogo;
use ratasvg::{build_svg_from_widget, svg};
use ratatui::widgets::{RatatuiLogo, RatatuiMascot};
use tui_big_text::{BigText, PixelSize};

use crate::header::Header;

mod header;

fn main() -> Result<(), Box<dyn Error>> {
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

    svg::save(
        "mousefood-logo.svg",
        &build_svg_from_widget(
            MouseFoodLogo,
            ratasvg::Options {
                background_color: "black",
                width_px: 1200,
                height_px: 400,
                font_size_px: 50,
            },
        ),
    )?;

    let big_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .lines(vec!["ratasvg".into(), "~~~~~".into()])
        .build();

    svg::save(
        "ratasvg.svg",
        &build_svg_from_widget(
            big_text,
            ratasvg::Options {
                background_color: "black",
                width_px: 1700,
                height_px: 400,
                font_size_px: 50,
            },
        ),
    )?;

    Ok(())
}
