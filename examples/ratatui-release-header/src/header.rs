//! Generates a terminal banner for Ratatui releases featuring a Ratatui logo, version info, and
//! a list of crates.
//!
//! Used for README.md, documentation, and release materials. Updated for every release starting
//! with v0.30.0 "Bryndza".
//!
//! This example runs with the Ratatui library code in the branch that you are currently
//! reading. See the [`latest`] branch for the code which works with the most recent Ratatui
//! release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest

use std::iter::zip;

use ratatui::{
    layout::{Flex, Spacing},
    prelude::*,
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Padding, Paragraph, RatatuiLogo},
};

const SEMVER: &str = "0.30.1";
const RELEASE_NAME: &str = "Bryndza";

const MAIN_DISHES: [&str; 4] = [
    "> ratatui",
    "> ratatui-core",
    "> ratatui-widgets",
    "> ratatui-macros",
];
const BACKENDS: [&str; 4] = [
    "> ratatui-crossterm",
    "> ratatui-termion",
    "> ratatui-termina",
    "> ratatui-termwiz",
];

const FG_COLOR: Color = Color::Rgb(246, 214, 187); // #F6D6BB
const BG_COLOR: Color = Color::Rgb(20, 20, 50); // #141432
const MENU_BORDER_COLOR: Color = Color::Rgb(255, 255, 160); // #FFFFA0

enum Rainbow {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

pub struct Header;

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, (FG_COLOR, BG_COLOR));

        let logo_width = 29;
        let menu_width = 23;
        let padding = 2; // Padding between logo and menu
        let menu_borders = 3;
        let height = MAIN_DISHES.len() as u16 + BACKENDS.len() as u16 + menu_borders;
        let width = logo_width + menu_width + padding;
        let center_area = area.centered(Constraint::Length(width), Constraint::Length(height));
        let layout =
            Layout::horizontal(Constraint::from_lengths([logo_width, padding, menu_width]));
        let [logo_area, _, menu_area] = center_area.layout(&layout);

        Logo.render(logo_area, buf);
        Menu.render(menu_area, buf);
    }
}

struct Logo;

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.inner(Margin::new(1, 0));
        let layout = Layout::vertical(Constraint::from_lengths([6, 2, 1])).flex(Flex::End);
        let [shadow_area, logo_area, version_area] = area.layout(&layout);

        // Divide the logo into letter sections for individual coloring, then render a block for each
        // letter with a color based on the row index.
        let letter_layout = Layout::horizontal(Constraint::from_lengths([5, 4, 4, 4, 4, 5, 1]));
        for (row_index, row) in shadow_area.rows().enumerate() {
            for (rainbow, letter_area) in zip(Rainbow::ROYGBIV, row.layout_vec(&letter_layout)) {
                let color = rainbow.gradient_color(row_index);
                Block::new().style(color).render(letter_area, buf);
            }
            RatatuiLogo::small().render(row, buf);
        }

        Block::new().style(FG_COLOR).render(logo_area, buf);
        RatatuiLogo::small().render(logo_area, buf);
        format!("v{SEMVER} \"{RELEASE_NAME}\"")
            .dim()
            .render(version_area, buf);
    }
}

impl Rainbow {
    const RED_GRADIENT: [u8; 6] = [41, 43, 50, 68, 104, 156];
    const GREEN_GRADIENT: [u8; 6] = [24, 30, 41, 65, 105, 168];
    const BLUE_GRADIENT: [u8; 6] = [55, 57, 62, 78, 113, 166];
    const AMBIENT_GRADIENT: [u8; 6] = [17, 18, 20, 25, 40, 60];

    const ROYGBIV: [Self; 7] = [
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Blue,
        Self::Indigo,
        Self::Violet,
    ];

    fn gradient_color(&self, row: usize) -> Color {
        let ambient = Self::AMBIENT_GRADIENT[row];
        let red = Self::RED_GRADIENT[row];
        let green = Self::GREEN_GRADIENT[row];
        let blue = Self::BLUE_GRADIENT[row];
        let blue_sat = Self::AMBIENT_GRADIENT[row].saturating_mul(6 - row as u8);
        let (r, g, b) = match self {
            Self::Red => (red, ambient, blue_sat),
            Self::Orange => (red, green / 2, blue_sat),
            Self::Yellow => (red, green, blue_sat),
            Self::Green => (ambient, green, blue_sat),
            Self::Blue => (ambient, ambient, blue.max(blue_sat)),
            Self::Indigo => (blue, ambient, blue.max(blue_sat)),
            Self::Violet => (red, ambient, blue.max(blue_sat)),
        };
        Color::Rgb(r, g, b)
    }
}

struct Menu;

impl Widget for Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical(Constraint::from_lengths([
            MAIN_DISHES.len() as u16 + 2,
            BACKENDS.len() as u16 + 2,
        ]))
        .spacing(Spacing::Overlap(1)); // Overlap to merge borders
        let [main_dishes_area, backends_area] = area.layout(&layout);

        MenuBlock {
            title: "Main Courses",
            menu_items: &MAIN_DISHES,
        }
        .render(main_dishes_area, buf);

        MenuBlock {
            title: "Pairings",
            menu_items: &BACKENDS,
        }
        .render(backends_area, buf)
    }
}

struct MenuBlock<'a> {
    title: &'a str,
    menu_items: &'a [&'a str],
}

impl Widget for MenuBlock<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let menu_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(MENU_BORDER_COLOR)
            .padding(Padding::horizontal(1))
            .merge_borders(MergeStrategy::Fuzzy)
            .title(self.title);

        let menu_lines: Vec<Line> = self
            .menu_items
            .iter()
            .map(|&item| Line::from(item))
            .collect();
        Paragraph::new(menu_lines)
            .block(menu_block)
            .render(area, buf);
    }
}
