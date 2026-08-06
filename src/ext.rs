
pub trait NumberExt {
    fn angle_diff(self, other: Self) -> Self;
}
macro_rules! impl_number_ext {
    (float, $($t:ty),*) => {
        $(
            impl NumberExt for $t {
                fn angle_diff(self, other: Self) -> Self {
                    let diff = (self - other).abs() % 360.0 as Self;

                    if diff > 180.0 {
                        360.0 - diff
                    } else {
                        diff
                    }
                }
            }
        )*
    };

    (int, $($t:ty),*) => {
        $(
            impl NumberExt for $t {
                fn angle_diff(self, other: Self) -> Self {
                    let diff = (self.abs_diff(other) % 360) as Self;

                    if diff > 180 {
                        360 - diff
                    } else {
                        diff
                    }
                }
            }
        )*
    };
}

impl_number_ext!(float, f32, f64);
impl_number_ext!(int, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
