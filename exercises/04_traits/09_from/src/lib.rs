// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

// impl From<i32> for WrappingU32 {
//     fn from(value: i32) -> Self {
//         WrappingU32{value: value as u32}
//     }
// }

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32{value}
    }
}

// impl<T, U> Into<U> for T
// where
//     U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
