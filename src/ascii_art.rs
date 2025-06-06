#[rustfmt::skip]
pub const ZERO: &[&str] = &[
    "█████",
    "█   █",
    "█   █",
    "█   █",
    "█████",
];

#[rustfmt::skip]
pub const ONE: &[&str] = &[
    "  █  ",
    "  █  ",
    "  █  ",
    "  █  ",
    "  █  ",
];

#[rustfmt::skip]
pub const TWO: &[&str] = &[
    "█████",
    "    █",
    "█████",
    "█    ",
    "█████",
];

#[rustfmt::skip]
pub const THREE: &[&str] = &[
    "█████",
    "    █",
    "█████",
    "    █",
    "█████",
];

#[rustfmt::skip]
pub const FOUR: &[&str] = &[
    "█   █",
    "█   █",
    "█████",
    "    █",
    "    █",
];

#[rustfmt::skip]
pub const FIVE: &[&str] = &[
    "█████",
    "█    ",
    "█████",
    "    █",
    "█████",
];

#[rustfmt::skip]
pub const SIX: &[&str] = &[
    "█████",
    "█    ",
    "█████",
    "█   █",
    "█████",
];

#[rustfmt::skip]
pub const SEVEN: &[&str] = &[
    "█████",
    "    █",
    "   █ ",
    "  █  ",
    "  █  ",
];

#[rustfmt::skip]
pub const EIGHT: &[&str] = &[
    "█████",
    "█   █",
    "█████",
    "█   █",
    "█████",
];

#[rustfmt::skip]
pub const NINE: &[&str] = &[
    "█████",
    "█   █",
    "█████",
    "    █",
    "█████",
];

#[rustfmt::skip]
pub const COLON: &[&str] = &[
    "     ",
    "  █  ",
    "     ",
    "  █  ",
    "     ",
];

pub const SYMBOL_DIMENSIONS: (usize, usize) = (5, 5);

pub const DIGITS: &[&[&str]] = &[ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];
