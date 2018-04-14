// Helper macros to generate register accessors
// Copyright 2018 Ryan Kurte

#[doc = "Write a value to the provided variable with the given mask and shift"]
#[macro_export]
macro_rules! write_masked {
    ($write: expr, $shift: expr, $mask: expr, $val: expr) => {
        $write = ($write & !($mask << $shift)) | ($val & $mask) << $shift
    }
}

#[doc = "Read a value from the provided variable with the given mask and shift"]
#[macro_export]
macro_rules! read_masked {
    ($read: expr, $shift: expr, $mask: expr) => {
        ($read >> $shift) & $mask
    }
}

#[doc = "Generates a trait for a provided field type"]
#[macro_export]
macro_rules! field_trait {
    (r, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(&self) -> bool;
    };
    (w, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(mut self, v: bool) -> Self;
    };
    (r, $name: ident, $field:tt, $t:ty, $shift:expr, $mask:expr) => {
        fn $name(&self) -> $t;
    };
    (w, $name: ident, $field:tt, $t:ty, $shift:expr, $mask:expr) => {
        fn $name(mut self, v: $t) -> Self;
    };
}

#[doc = "Generates a method for a provided field type"]
#[macro_export]
macro_rules! field_method {
    (r, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(&self) -> bool {
            self.$field & (1 << $shift) != 0
        }
    };
    (w, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(mut self, v: bool) -> Self {
            self.$field = match v {
                true => self.$field | (1 << $shift),
                false => self.$field & !(1 << $shift),
            };
            self
        }
    };
    (r, $name: ident, $field: tt, $t:ty, $shift: expr, $mask: expr) => {
        fn $name(&self) -> $t {
            read_masked!(self.$field, $shift, $mask)
        }
    };
    (w, $name: ident, $field: tt, $t:ty, $shift: expr, $mask: expr) => {
        fn $name(mut self, v: $t) -> Self{
            write_masked!(self.$field, $shift, $mask, v);
            self
        }
    };
}

#[doc = "Creates accessor traits and implementations for a given register\n"]
#[doc = "`register!(name, type, [r/w/rw, field name, field in object (ie. 1 for register tuple), return type, (mask for non-bool types)];`"]
#[macro_export]
macro_rules! register {
    (
        $reg:ident, $t:ty, [ $( $op:ident, $name:ident, $field:tt, $type:ty, $( $args:expr ),* );* ;]
    ) => {
        pub trait $reg {
            $( field_trait!($op, $name, $field, $type, $( $args ),* ); )*
        }
        impl $reg for Register<$t> {
            $( field_method!($op, $name, $field, $type, $( $args ),* ); )*
        }
    }
}

#[cfg(test)]
mod tests {
    use ::register::Register;

    register!(TESTREG1, u16, 
        [
            r, read_bit1,   1,  bool,   1;
            w, write_bit1,  1,  bool,   1;
            r, read_var1,   1,  u16,    2,  0b111;
            w, write_var1,  1,  u16,    2,  0b111;
        ]
    );

    #[test]
    fn register_traits() {
        let mut r = Register::<u16>(0, 0);

        assert_eq!(0, r.value());
        assert_eq!(false, r.read_bit1());
        r.write_bit1(true);
        assert_eq!(true, r.read_bit1());
        assert_eq!(1 << 1, r.value());

        assert_eq!(0, TESTREG1::read_var1(&r));
        TESTREG1::write_var1(&mut r, 3);
        assert_eq!(3, TESTREG1::read_var1(&r));

        assert_eq!(1 << 1 | 3 << 2, r.value());
    }
}

