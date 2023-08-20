use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};
use once_cell::sync::Lazy;

pub static RENDER_CONFIG: Lazy<RenderConfig> = Lazy::new(|| {
    let mut config = RenderConfig::default()
        .with_text_input(
            StyleSheet::new()
                .with_fg(Color::LightCyan)
                .with_attr(Attributes::BOLD),
        )
        .with_prompt_prefix(Styled::new(">").with_fg(Color::DarkGrey))
        .with_default_value(
            StyleSheet::new()
                .with_fg(Color::LightMagenta)
                .with_attr(Attributes::ITALIC),
        )
        .with_answer(
            StyleSheet::new()
                .with_fg(Color::LightCyan)
                .with_attr(Attributes::ITALIC),
        )
        .with_selected_option(Some(
            StyleSheet::new()
                .with_fg(Color::LightCyan)
                .with_attr(Attributes::BOLD),
        ))
        .with_highlighted_option_prefix(
            Styled::new(">")
                .with_fg(Color::LightCyan)
                .with_attr(Attributes::BOLD),
        );

    config.answered_prompt_prefix = Styled::new(">").with_fg(Color::LightMagenta);

    config
});
