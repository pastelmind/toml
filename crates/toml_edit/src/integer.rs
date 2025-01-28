use std::error;
use std::fmt;

/// Type representing a TOML integer,
/// payload of the `Value::Integer` variant
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Integer(IntegerImpl);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum IntegerImpl {
    /// Decimal integer. Can be positive or negative.
    Decimal(i64),
    /// Hexadecimal integer. Always nonnegative.
    Hexadecimal(i64, HexadecimalCase),
    /// Octal integer. Always nonnegative.
    Octal(i64),
    /// Binary integer. Always nonnegative.
    Binary(i64),
}

/// Describes the preferred case of alphabets in the default raw representation
/// of a hexadecimal integer.
/// Note that the raw representation of a [`Formatted<Integer>`][crate::repr::Formatted]
/// parsed from TOML is not affected by this.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum HexadecimalCase {
    /// Uppercase hexadecimal integers.
    Uppercase,
    /// Lowercase hexadecimal integers.
    Lowercase,
    /// Reserved for [Integer]s parsed from TOML strings.
    Unspecified,
}

impl Integer {
    /// Create a new `Integer` as decimal notation.
    pub fn new(value: i64) -> Self {
        Self(IntegerImpl::Decimal(value))
    }

    /// Attempts to create a new `Integer` as hexadecimal notation in uppercase.
    /// Fails if `value` is negative.
    pub fn new_hex_upper(value: i64) -> Result<Self, IntegerError> {
        if value >= 0 {
            Ok(Self(IntegerImpl::Hexadecimal(
                value,
                HexadecimalCase::Uppercase,
            )))
        } else {
            Err(IntegerError {})
        }
    }

    /// Attempts to create a new `Integer` as hexadecimal notation in lowercase.
    /// Fails if `value` is negative.
    pub fn new_hex_lower(value: i64) -> Result<Self, IntegerError> {
        if value >= 0 {
            Ok(Self(IntegerImpl::Hexadecimal(
                value,
                HexadecimalCase::Lowercase,
            )))
        } else {
            Err(IntegerError {})
        }
    }

    /// Attempts to create a new `Integer` as hexadecimal notation with
    /// unspecified case.
    /// Fails if `value` is negative.
    pub(crate) fn new_hex_unspecified(value: i64) -> Result<Self, IntegerError> {
        if value >= 0 {
            Ok(Self(IntegerImpl::Hexadecimal(
                value,
                HexadecimalCase::Unspecified,
            )))
        } else {
            Err(IntegerError {})
        }
    }

    /// Attempts to create a new `Integer` as octal notation.
    /// Fails if `value` is negative.
    pub fn new_oct(value: i64) -> Result<Self, IntegerError> {
        if value >= 0 {
            Ok(Self(IntegerImpl::Octal(value)))
        } else {
            Err(IntegerError {})
        }
    }

    /// Attempts to create a new `Integer` as binary notation.
    /// Fails if `value` is negative.
    pub fn new_bin(value: i64) -> Result<Self, IntegerError> {
        if value >= 0 {
            Ok(Self(IntegerImpl::Binary(value)))
        } else {
            Err(IntegerError {})
        }
    }

    /// Returns the value of the integer.
    pub fn value(&self) -> i64 {
        match self.0 {
            IntegerImpl::Decimal(value) => value,
            IntegerImpl::Hexadecimal(value, ..) => value,
            IntegerImpl::Octal(value) => value,
            IntegerImpl::Binary(value) => value,
        }
    }

    /// Returns true if this integer is decimal.
    pub fn is_dec(&self) -> bool {
        matches!(self.0, IntegerImpl::Decimal(..))
    }

    /// Returns true if this integer is hexadecimal.
    pub fn is_hex(&self) -> bool {
        matches!(self.0, IntegerImpl::Hexadecimal(..))
    }

    /// Returns true if this integer is octal.
    pub fn is_oct(&self) -> bool {
        matches!(self.0, IntegerImpl::Octal(..))
    }

    /// Returns true if this integer is binary.
    pub fn is_bin(&self) -> bool {
        matches!(self.0, IntegerImpl::Binary(..))
    }

    /// Creates a string representation using the current notation.
    pub fn to_string(&self) -> String {
        match self.0 {
            IntegerImpl::Decimal(v) => v.to_string(),
            IntegerImpl::Hexadecimal(v, case) => match case {
                HexadecimalCase::Uppercase => format!("{:#X}", v),
                HexadecimalCase::Lowercase => format!("{:#x}", v),
                HexadecimalCase::Unspecified => format!("{:#X}", v),
            },
            IntegerImpl::Octal(v) => format!("{:#o}", v),
            IntegerImpl::Binary(v) => format!("{:#b}", v),
        }
    }
}

impl From<Integer> for i64 {
    fn from(i: Integer) -> i64 {
        i.value()
    }
}

impl From<&Integer> for i64 {
    fn from(i: &Integer) -> i64 {
        i.value()
    }
}

/// Error returned when creating an invalid `Integer`.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct IntegerError {}

impl fmt::Display for IntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "invalid integer".fmt(f)
    }
}

impl error::Error for IntegerError {}
