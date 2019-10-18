use tui::style::{Color, Style};

fn default_style() -> Style {
    Style::default().fg(Color::LightMagenta)
}

fn active_style() -> Style {
    Style::default().fg(Color::Green)
}

pub fn style(active: bool) -> Style {
    if active {
        active_style()
    } else {
        default_style()
    }
}
