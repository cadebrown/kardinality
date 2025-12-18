#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiTheme {
    Crt,
    Terminal,
    Magic,
}

impl UiTheme {
    pub fn label(&self) -> &'static str {
        match self {
            UiTheme::Crt => "CRT",
            UiTheme::Terminal => "Terminal",
            UiTheme::Magic => "Arcane",
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            UiTheme::Crt => "theme-crt",
            UiTheme::Terminal => "theme-terminal",
            UiTheme::Magic => "theme-magic",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiSettings {
    pub theme: UiTheme,
    pub effects: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: UiTheme::Crt,
            // Effects are awesome, but default OFF for now so the baseline UI is stable/readable.
            effects: false,
        }
    }
}
