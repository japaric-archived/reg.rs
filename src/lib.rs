//! Type safe registers

#![feature(core)]
#![feature(no_std)]
#![no_std]

extern crate core;

#[macro_export]
macro_rules! reg {
    ($register:ident: $ty:ident {
        $($bit:ident: $nth:expr),+,
    }) => {
        reg!($register: $ty {
            bits {
                $($bit: $nth),+,
            }
            bitfields {}
        });
    };

    ($register:ident: $ty:ident {
        bits {
            $($bit:ident: $nth:expr),+,
        }
        bitfields {
            $($bitfield:ident: $offset:expr { $($state:ident: $value:expr),+, }),*
        }
    }) => {
        pub mod $register {
            pub mod prelude {
                pub use super::Bit::*;
                pub use super::{$($bitfield),*};
            }

            use core::ops::{BitAnd, BitOr, Not};

            use volatile::Into;

            #[derive(Clone, Copy)]
            #[repr(C)]
            pub struct Register($ty);

            pub enum Bit {
                $($bit,)+
            }

            impl Bit {
                fn $ty(&self) -> $ty {
                    use self::Bit::*;

                    match *self {
                        $($bit => 1 << $nth),+
                    }
                }
            }

            impl BitAnd<$crate::Not<Bit>> for Register {
                type Output = Register;

                fn bitand(self, rhs: $crate::Not<Bit>) -> Register {
                    Register(self.0 & !(rhs.into_inner()).$ty())
                }
            }

            impl BitOr for Bit {
                type Output = Register;

                fn bitor(self, rhs: Bit) -> Register {
                    Register(self.$ty() | rhs.$ty())
                }
            }

            impl BitOr<Bit> for Register {
                type Output = Register;

                fn bitor(mut self, rhs: Bit) -> Register {
                    self.0 |= rhs.$ty();
                    self
                }
            }

            impl BitOr<Register> for Bit {
                type Output = Register;

                fn bitor(self, rhs: Register) -> Register {
                    rhs | self
                }
            }

            impl Not for Bit {
                type Output = $crate::Not<Bit>;

                fn not(self) -> $crate::Not<Bit> {
                    $crate::Not::new(self)
                }
            }

            impl Into<Register> for Bit {
                fn convert_into(self) -> Register {
                    Register(self.$ty())
                }
            }

            $(
                pub enum $bitfield {
                    $($state),+,
                }

                impl $bitfield {
                    fn $ty(&self) -> $ty {
                        use self::$bitfield::*;

                        match *self {
                            $($state => $value << $offset),+
                        }
                    }
                }

                impl BitOr<$bitfield> for Register {
                    type Output = Register;

                    fn bitor(mut self, rhs: $bitfield) -> Register {
                        const MASK: $ty = ($($value)|+) << $offset;

                        self.0 &= !MASK;
                        self.0 |= rhs.$ty();
                        self
                    }
                }

                impl BitOr<Register> for $bitfield{
                    type Output = Register;

                    fn bitor(self, rhs: Register) -> Register {
                        rhs | self
                    }
                }

                impl BitOr<$bitfield> for Bit {
                    type Output = Register;

                    fn bitor(self, rhs: $bitfield) -> Register {
                        Register(self.$ty() | rhs.$ty())
                    }
                }

                impl BitOr<Bit> for $bitfield {
                    type Output = Register;

                    fn bitor(self, rhs: Bit) -> Register {
                        rhs | self
                    }
                }
             )*
        }
    };

    ($register:ident: $ty:ident {
        bitfields {
            $($bitfield:ident: $offset:expr { $($state:ident: $value:expr),+, }),+,
        }
    }) => {
        pub mod $register {
            #![allow(non_camel_case_types)]

            pub mod prelude {
                pub use super::{$($bitfield),+};
            }

            use core::ops::BitOr;

            #[derive(Clone, Copy)]
            #[repr(C)]
            pub struct Register($ty);

            $(
                pub enum $bitfield {
                    $($state),+,
                }

                impl $bitfield {
                    fn $ty(&self) -> $ty {
                        use self::$bitfield::*;

                        match *self {
                            $($state => $value << $offset),+
                        }
                    }
                }

                impl BitOr<$bitfield> for Register {
                    type Output = Register;

                    fn bitor(mut self, rhs: $bitfield) -> Register {
                        const MASK: $ty = ($($value)|+) << $offset;

                        self.0 &= !MASK;
                        self.0 |= rhs.$ty();
                        self
                    }
                }

                impl BitOr<Register> for $bitfield{
                    type Output = Register;

                    fn bitor(self, rhs: Register) -> Register {
                        rhs | self
                    }
                }
             )+
        }
    };
}

/// Lazy bitwise negation `!x`
pub struct Not<T>(T);

impl<T> Not<T> {
    #[doc(hidden)]
    pub fn new(bit: T) -> Not<T> {
        Not(bit)
    }

    #[doc(hidden)]
    pub fn into_inner(self) -> T {
        self.0
    }
}
