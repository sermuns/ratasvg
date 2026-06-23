use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};

fn main() {
    let area = Rect {
        width: 10,
        height: 10,
        ..Default::default()
    };
    let mut buf = Buffer::empty(area);

    render(area, &mut buf);

    dbg!(buf);
}

fn render(area: Rect, buf: &mut Buffer) {
    "fuck".render(area, buf);
}
