pub trait Bounded: Sized {
    const MIN: Self;
    const MAX: Self;
}

impl Bounded for char {
    const MIN: char = '\0';
    const MAX: char = char::MAX;
}

macro_rules! impl_int {
	($($ty:ident),*) => {
		$(
			impl Bounded for $ty {
				const MIN: Self = $ty::MIN;
				const MAX: Self = $ty::MAX;
			}
		)*
	};
}

impl_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_float {
	($($ty:ident),*) => {
		$(
			impl Bounded for $ty {
				const MIN: Self = $ty::NEG_INFINITY;
				const MAX: Self = $ty::INFINITY;
			}
		)*
	};
}

impl_float!(f32, f64);
