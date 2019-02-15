use arrayvec::ArrayString;

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN, MAX_DEC_LEN, MAX_SEP_LEN};
use crate::utils::{InfinityStr, MinusSignStr, NanStr, DecimalStr, SeparatorStr};
use crate::{CustomFormatBuilder, Format, Grouping, Locale};

/// Type for representing your own custom formats. Implements [`Format`].
///
/// # Example
/// ```rust
/// use num_format::{Buffer, Error, CustomFormat, Grouping};
///
/// fn main() -> Result<(), Error> {
///     let format = CustomFormat::builder()
///         .grouping(Grouping::Indian)
///         .minus_sign("🙌")
///         .separator(Some('😀'))
///         .build()?;
///
///     let mut buf = Buffer::new();
///     buf.write_formatted(&(-1000000), &format);
///     assert_eq!("🙌10😀00😀000", buf.as_str());
///
///     Ok(())
/// }
/// ```
///
/// [`Format`]: trait.Format.html
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CustomFormat {
    pub(crate) dec: ArrayString<[u8; MAX_DEC_LEN]>,
    pub(crate) grp: Grouping,
    pub(crate) inf: ArrayString<[u8; MAX_INF_LEN]>,
    pub(crate) min: ArrayString<[u8; MAX_MIN_LEN]>,
    pub(crate) nan: ArrayString<[u8; MAX_NAN_LEN]>,
    pub(crate) sep: ArrayString<[u8; MAX_SEP_LEN]>,
}

impl CustomFormat {
    /// Constructs a [`CustomFormatBuilder`].
    ///
    /// [`CustomFormatBuilder`]: struct.CustomFormatBuilder.html
    pub fn builder() -> CustomFormatBuilder {
        CustomFormatBuilder::new()
    }

    /// Turns `self` into a [`CustomFormatBuilder`].
    ///
    /// [`CustomFormatBuilder`]: struct.CustomFormatBuilder.html
    pub fn into_builder(self) -> CustomFormatBuilder {
        self.into()
    }

    /// Returns this format's representation of decimal points.
    pub fn decimal(&self) -> &str {
        &self.dec
    }

    /// Returns this format's [`Grouping`], which governs how digits are separated (see [`Grouping`]).
    ///
    /// [`Grouping`]: enum.Grouping.html
    pub fn grouping(&self) -> Grouping {
        self.grp
    }

    /// Returns this format's representation of infinity.
    pub fn infinity(&self) -> &str {
        &self.inf
    }

    /// Returns this format's representation of minus signs.
    pub fn minus_sign(&self) -> &str {
        &self.min
    }

    /// Returns this format's representation of NaN.
    pub fn nan(&self) -> &str {
        &self.nan
    }

    /// Returns this format's representation of separators.
    pub fn separator(&self) -> &str {
        &self.sep
    }
}

impl Format for CustomFormat {
    fn decimal(&self) -> DecimalStr<'_> {
        DecimalStr::new(self.decimal()).unwrap()
    }

    fn grouping(&self) -> Grouping {
        self.grouping()
    }

    fn infinity(&self) -> InfinityStr<'_> {
        InfinityStr::new(self.infinity()).unwrap()
    }

    fn minus_sign(&self) -> MinusSignStr<'_> {
        MinusSignStr::new(self.minus_sign()).unwrap()
    }

    fn nan(&self) -> NanStr<'_> {
        NanStr::new(self.nan()).unwrap()
    }

    fn separator(&self) -> SeparatorStr<'_> {
        SeparatorStr::new(self.separator()).unwrap()
    }
}

impl From<Locale> for CustomFormat {
    fn from(locale: Locale) -> Self {
        Self {
            dec: ArrayString::from(locale.decimal()).unwrap(),
            grp: locale.grouping(),
            inf: ArrayString::from(locale.infinity()).unwrap(),
            min: ArrayString::from(locale.minus_sign()).unwrap(),
            nan: ArrayString::from(locale.nan()).unwrap(),
            sep: ArrayString::from(locale.separator()).unwrap(),
        }
    }
}

#[cfg(all(feature = "system", any(unix, windows)))]
mod system {
    use arrayvec::ArrayString;

    use crate::{CustomFormat, SystemLocale};

    impl From<SystemLocale> for CustomFormat {
        fn from(locale: SystemLocale) -> Self {
            Self {
                dec: ArrayString::from(locale.decimal()).unwrap(),
                grp: locale.grouping(),
                inf: ArrayString::from(locale.infinity()).unwrap(),
                min: ArrayString::from(locale.minus_sign()).unwrap(),
                nan: ArrayString::from(locale.nan()).unwrap(),
                sep: ArrayString::from(locale.separator()).unwrap(),
            }
        }
    }
}
