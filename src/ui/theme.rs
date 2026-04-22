use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub accent_secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub muted: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub footer_bg: Color,
    pub footer_fg: Color,
    pub category_fg: Color,
}

impl Theme {
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".into(),
            bg: Color::Rgb(40, 42, 54),
            fg: Color::Rgb(248, 248, 242),
            accent: Color::Rgb(189, 147, 249),           // Purple
            accent_secondary: Color::Rgb(255, 121, 198), // Pink
            success: Color::Rgb(80, 250, 123),           // Green
            warning: Color::Rgb(241, 250, 140),          // Yellow
            error: Color::Rgb(255, 85, 85),              // Red
            muted: Color::Rgb(98, 114, 164),             // Comment
            selection_bg: Color::Rgb(68, 71, 90),
            selection_fg: Color::Rgb(248, 248, 242),
            footer_bg: Color::Rgb(189, 147, 249),
            footer_fg: Color::Rgb(40, 42, 54),
            category_fg: Color::Rgb(139, 233, 253), // Cyan
        }
    }

    pub fn catppuccin() -> Self {
        Self {
            name: "Catppuccin".into(),
            bg: Color::Rgb(30, 30, 46),
            fg: Color::Rgb(205, 214, 244),
            accent: Color::Rgb(137, 180, 250),           // Blue
            accent_secondary: Color::Rgb(203, 166, 247), // Mauve
            success: Color::Rgb(166, 227, 161),          // Green
            warning: Color::Rgb(249, 226, 175),          // Yellow
            error: Color::Rgb(243, 139, 168),            // Red
            muted: Color::Rgb(108, 112, 134),            // Overlay
            selection_bg: Color::Rgb(49, 50, 68),
            selection_fg: Color::Rgb(205, 214, 244),
            footer_bg: Color::Rgb(180, 190, 254),
            footer_fg: Color::Rgb(30, 30, 46),
            category_fg: Color::Rgb(148, 226, 213), // Teal
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".into(),
            bg: Color::Rgb(46, 52, 64),
            fg: Color::Rgb(236, 239, 244),
            accent: Color::Rgb(136, 192, 208), // Frost
            accent_secondary: Color::Rgb(129, 161, 193),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            error: Color::Rgb(191, 97, 106),
            muted: Color::Rgb(76, 86, 106),
            selection_bg: Color::Rgb(59, 66, 82),
            selection_fg: Color::Rgb(236, 239, 244),
            footer_bg: Color::Rgb(129, 161, 193),
            footer_fg: Color::Rgb(46, 52, 64),
            category_fg: Color::Rgb(143, 188, 187),
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name: "Gruvbox".into(),
            bg: Color::Rgb(40, 40, 40),
            fg: Color::Rgb(235, 219, 178),
            accent: Color::Rgb(215, 153, 33),          // Gold
            accent_secondary: Color::Rgb(214, 93, 14), // Orange
            success: Color::Rgb(152, 151, 26),
            warning: Color::Rgb(250, 189, 47),
            error: Color::Rgb(204, 36, 29),
            muted: Color::Rgb(146, 131, 116),
            selection_bg: Color::Rgb(60, 56, 54),
            selection_fg: Color::Rgb(235, 219, 178),
            footer_bg: Color::Rgb(215, 153, 33),
            footer_fg: Color::Rgb(40, 40, 40),
            category_fg: Color::Rgb(104, 157, 106),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".into(),
            bg: Color::Rgb(26, 27, 38),
            fg: Color::Rgb(169, 177, 214),
            accent: Color::Rgb(122, 162, 247),           // Blue
            accent_secondary: Color::Rgb(187, 154, 247), // Purple
            success: Color::Rgb(158, 206, 106),
            warning: Color::Rgb(224, 175, 104),
            error: Color::Rgb(247, 118, 142),
            muted: Color::Rgb(86, 95, 137),
            selection_bg: Color::Rgb(41, 46, 66),
            selection_fg: Color::Rgb(169, 177, 214),
            footer_bg: Color::Rgb(122, 162, 247),
            footer_fg: Color::Rgb(26, 27, 38),
            category_fg: Color::Rgb(42, 195, 222),
        }
    }

    pub fn solarized() -> Self {
        Self {
            name: "Solarized".into(),
            bg: Color::Rgb(0, 43, 54),
            fg: Color::Rgb(131, 148, 150),
            accent: Color::Rgb(38, 139, 210),            // Blue
            accent_secondary: Color::Rgb(108, 113, 196), // Violet
            success: Color::Rgb(133, 153, 0),
            warning: Color::Rgb(181, 137, 0),
            error: Color::Rgb(220, 50, 47),
            muted: Color::Rgb(88, 110, 117),
            selection_bg: Color::Rgb(7, 54, 66),
            selection_fg: Color::Rgb(147, 161, 161),
            footer_bg: Color::Rgb(38, 139, 210),
            footer_fg: Color::Rgb(0, 43, 54),
            category_fg: Color::Rgb(42, 161, 152),
        }
    }

    pub fn minimal() -> Self {
        Self {
            name: "Minimal".into(),
            bg: Color::Reset,
            fg: Color::White,
            accent: Color::Yellow,
            accent_secondary: Color::Cyan,
            success: Color::Green,
            warning: Color::Rgb(255, 255, 0),
            error: Color::Red,
            muted: Color::Gray,
            selection_bg: Color::Rgb(40, 40, 40),
            selection_fg: Color::White,
            footer_bg: Color::Blue,
            footer_fg: Color::White,
            category_fg: Color::Magenta,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::minimal(),
            Self::dracula(),
            Self::catppuccin(),
            Self::nord(),
            Self::gruvbox(),
            Self::tokyo_night(),
            Self::solarized(),
        ]
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::minimal()
    }
}
