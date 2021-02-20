use crate::palette::{BaseScale, Palette};
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("editor.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("foreground", palette.base(BaseScale::Fg));

    builder.add_workspace_rule(
        "editorLineNumber.foreground",
        palette.base(BaseScale::BarelyVisibleFg),
    );
    builder.add_workspace_rule("editorGutter.background", palette.base(BaseScale::MiddleBg));

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::MiddleBg),
    );

    builder.add_workspace_rule("sideBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule(
        "sideBarSectionHeader.background",
        palette.base(BaseScale::DarkBg),
    );
    builder.add_workspace_rule(
        "sideBarSectionHeader.foreground",
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rule("activityBar.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("activityBar.foreground", palette.base(BaseScale::BrightFg));
    builder.add_workspace_rule(
        "activityBar.inactiveForeground",
        palette.base(BaseScale::BarelyVisibleFg),
    );

    builder.add_workspace_rule(
        "editorGroupHeader.tabsBackground",
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rule("tab.inactiveBackground", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("tab.inactiveForeground", palette.base(BaseScale::DimmedFg));
    builder.add_workspace_rule("tab.activeBackground", palette.base(BaseScale::MiddleBg));
    builder.add_workspace_rule("tab.activeForeground", palette.base(BaseScale::BrightFg));

    builder.add_workspace_rule("statusBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::DimmedFg));

    builder.add_workspace_rule(
        "editor.selectionBackground",
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule("selection.background", palette.base(BaseScale::LightBg));

    builder.add_workspace_rule(
        "editorCodeLens.foreground",
        palette.base(BaseScale::DimmedFg),
    );
    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DimmedFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.base(BaseScale::FadedFg));

    builder.add_rule(Semantic("comment"), palette.base(BaseScale::BrightFg));

    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
            Semantic("type.declaration"),
            Semantic("class.declaration"),
            Semantic("struct.declaration"),
            Semantic("enum.declaration"),
            Semantic("union.declaration"),
            Semantic("typeAlias.declaration"),
            Semantic("interface.declaration"),
            Semantic("namespace.declaration"),
        ],
        palette.orange(),
    );

    builder.add_rule(Semantic("string"), palette.cyan());

    builder.add_rules(
        &[
            Semantic("number"),
            Semantic("boolean"),
            Semantic("formatSpecifier"),
            Semantic("enumMember"),
            Semantic("*.constant"),
        ],
        palette.purple(),
    );
}
