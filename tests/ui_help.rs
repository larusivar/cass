use coding_agent_search::ui::tui::footer_legend;

#[test]
fn help_legend_has_hotkeys() {
    let short = footer_legend(false);
    assert!(short.contains("F1 help"));
    assert!(short.contains("F11 clear"));
    let long = footer_legend(true);
    assert!(long.contains("Esc/F10 quit"));
    assert!(long.contains("F7 context"));
}
