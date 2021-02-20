use crate::palette::{BaseScale, Palette};
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("editor.foreground", palette.fg());
    builder.add_workspace_rule("foreground", palette.fg());
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.base(BaseScale::FadedFg));

    builder.add_rule(Semantic("comment"), palette.bright_fg());
}
