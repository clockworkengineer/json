//! JSON syntax character constants
//!
//! Centralizes all character literals used across the JSON parser modules
//! so they are defined in one place and reused consistently.

/// JSON object delimiters
pub const OBJECT_START: char = '{';
pub const OBJECT_END: char = '}';

/// JSON array delimiters
pub const ARRAY_START: char = '[';
pub const ARRAY_END: char = ']';

/// JSON string delimiter
pub const QUOTE: char = '"';

/// JSON value separator
pub const COMMA: char = ',';

/// JSON key-value separator
pub const COLON: char = ':';

/// JSON string escape character
pub const BACKSLASH: char = '\\';

/// Starting characters for JSON literal values
pub const TRUE_START: char = 't';
pub const FALSE_START: char = 'f';
pub const NULL_START: char = 'n';

/// Number literal characters
pub const MINUS: char = '-';
pub const PLUS: char = '+';
pub const DECIMAL_POINT: char = '.';
pub const EXPONENT_LOWER: char = 'e';
pub const EXPONENT_UPPER: char = 'E';

/// Whitespace characters
pub const SPACE: char = ' ';
pub const TAB: char = '\t';
pub const NEWLINE: char = '\n';
pub const CARRIAGE_RETURN: char = '\r';
