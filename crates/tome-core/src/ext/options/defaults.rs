//! Default editor options.

use linkme::distributed_slice;

use crate::ext::options::{OptionDef, OptionScope, OptionType, OptionValue, OPTIONS};

#[distributed_slice(OPTIONS)]
static OPT_TAB_WIDTH: OptionDef = OptionDef {
    name: "tab_width",
    description: "Width of a tab character",
    value_type: OptionType::Int,
    default: || OptionValue::Int(4),
    scope: OptionScope::Buffer,
};

#[distributed_slice(OPTIONS)]
static OPT_INDENT_WIDTH: OptionDef = OptionDef {
    name: "indent_width",
    description: "Number of spaces for each indent level",
    value_type: OptionType::Int,
    default: || OptionValue::Int(4),
    scope: OptionScope::Buffer,
};

#[distributed_slice(OPTIONS)]
static OPT_USE_TABS: OptionDef = OptionDef {
    name: "use_tabs",
    description: "Use tabs instead of spaces for indentation",
    value_type: OptionType::Bool,
    default: || OptionValue::Bool(false),
    scope: OptionScope::Buffer,
};

#[distributed_slice(OPTIONS)]
static OPT_LINE_NUMBERS: OptionDef = OptionDef {
    name: "line_numbers",
    description: "Show line numbers in the gutter",
    value_type: OptionType::Bool,
    default: || OptionValue::Bool(true),
    scope: OptionScope::Global,
};

#[distributed_slice(OPTIONS)]
static OPT_WRAP_LINES: OptionDef = OptionDef {
    name: "wrap_lines",
    description: "Wrap long lines",
    value_type: OptionType::Bool,
    default: || OptionValue::Bool(true),
    scope: OptionScope::Buffer,
};

#[distributed_slice(OPTIONS)]
static OPT_SCROLL_MARGIN: OptionDef = OptionDef {
    name: "scroll_margin",
    description: "Minimum lines to keep above/below cursor when scrolling",
    value_type: OptionType::Int,
    default: || OptionValue::Int(3),
    scope: OptionScope::Global,
};
