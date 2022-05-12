/// Type for which instances can be partially enumerated.
pub trait PartialEnum: Sized + PartialOrd {
    fn pred(&self) -> Option<Self>;

    fn succ(&self) -> Option<Self>;
}

/// Type for which instances can be entirely enumerated.
pub trait Enum: Sized + PartialOrd {
    fn pred(&self) -> Option<Self>;

    fn succ(&self) -> Option<Self>;
}

impl<T: Enum> PartialEnum for T {
    fn pred(&self) -> Option<Self> {
        self.pred()
    }

    fn succ(&self) -> Option<Self> {
        self.succ()
    }
}

impl Enum for char {
    fn pred(&self) -> Option<Self> {
        match self {
            '\u{0000}' => None,
            '\u{e000}' => Some('\u{d7ff}'),
            _ => Some(unsafe { std::char::from_u32_unchecked(*self as u32 - 1) }),
        }
    }

    fn succ(&self) -> Option<Self> {
        match self {
            '\u{10ffff}' => None,
            '\u{d7ff}' => Some('\u{e000}'),
            _ => Some(unsafe { std::char::from_u32_unchecked(*self as u32 + 1) }),
        }
    }
}

macro_rules! impl_int {
	($($ty:ident),*) => {
		$(
			impl Enum for $ty {
				fn pred(&self) -> Option<Self> {
					self.checked_add(1)
				}

				fn succ(&self) -> Option<Self> {
					self.checked_sub(1)
				}
			}
		)*
	};
}

impl_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_float {
	($($ty:ident),*) => {
		$(
			impl PartialEnum for $ty {
				fn pred(&self) -> Option<Self> {
					None
				}

				fn succ(&self) -> Option<Self> {
					None
				}
			}
		)*
	};
}

impl_float!(f32, f64);
