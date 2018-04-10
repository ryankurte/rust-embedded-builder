// Helper macros to generate register interfaces

// Writes a value to the provided variable with the given mask and shift
#[macro_export]
macro_rules! write_masked {
    ($write: expr, $shift: expr, $mask: expr, $val: expr) => {
        $write = ($write & !($mask << $shift)) | ($val & $mask) << $shift
    }
}

// Reads a value from the provided variable with the given mask and shift
#[macro_export]
macro_rules! read_masked {
    ($read: expr, $shift: expr, $mask: expr) => {
        ($read >> $shift) & $mask
    }
}

// Generates a trait for a provided field type
#[macro_export]
macro_rules! field_trait {
    (read, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(&self) -> bool;
    };
    (write, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(&mut self, v: bool);
    };
    (read, $name: ident, $field:tt, $t:ty, $shift:expr, $mask:expr) => {
        fn $name(&self) -> $t;
    };
    (write, $name: ident, $field:tt, $t:ty, $shift:expr, $mask:expr) => {
        fn $name(&mut self, v: $t);
    }
}

// Generates a method for a provided field type
#[macro_export]
macro_rules! field_method {
    (read, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(&self) -> bool {
            self.$field & (1 << $shift) != 0
        }
    };
    (write, $name: ident, $field: tt, $t:ty, $shift: expr) => {
        fn $name(&mut self, v: bool) {
            self.$field = match v {
                true => self.$field | (1 << $shift),
                false => self.$field & !(1 << $shift),
            };
        }
    };
    (read, $name: ident, $field: tt, $t:ty, $shift: expr, $mask: expr) => {
        fn $name(&self) -> $t {
            read_masked!(self.$field, $shift, $mask)
        }
    };
    (write, $name: ident, $field: tt, $t:ty, $shift: expr, $mask: expr) => {
        fn $name(&mut self, v: $t) {
            write_masked!(self.$field, $shift, $mask, v);
        }
    }
}

// Creates a register with the specified fields
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
            read,   read_bit1,  1,  bool,   1;
            write,  write_bit1, 1,  bool,   1;
            read,   read_var,   1,  u16,    2,  0b111;
            write,  write_var,  1,  u16,    2,  0b111;
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

        assert_eq!(0, TESTREG1::read_var(&r));
        TESTREG1::write_var(&mut r, 3);
        assert_eq!(3, TESTREG1::read_var(&r));

        assert_eq!(1 << 1 | 3 << 2, r.value());
    }
}

