use crate::value::Value;

#[derive(Clone, Debug)]
pub enum Numeric {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    USize(usize),
    ISize(isize),
}


macro_rules! impl_math {
    ($name:ident, $op:tt) => {
        pub fn $name(&self, rhs: &Self) -> Option<Value> {
            match (self, rhs) {
                (Numeric::UInt8(a),   Numeric::UInt8(b)  ) => Some(Value::Numeric(Numeric::UInt8(a $op b))),
                (Numeric::UInt16(a),  Numeric::UInt16(b) ) => Some(Value::Numeric(Numeric::UInt16(a $op b))),
                (Numeric::UInt32(a),  Numeric::UInt32(b) ) => Some(Value::Numeric(Numeric::UInt32(a $op b))),
                (Numeric::UInt64(a),  Numeric::UInt64(b) ) => Some(Value::Numeric(Numeric::UInt64(a $op b))),
                (Numeric::UInt128(a), Numeric::UInt128(b)) => Some(Value::Numeric(Numeric::UInt128(a $op b))),
                (Numeric::Int8(a),    Numeric::Int8(b)   ) => Some(Value::Numeric(Numeric::Int8(a $op b))),
                (Numeric::Int16(a),   Numeric::Int16(b)  ) => Some(Value::Numeric(Numeric::Int16(a $op b))),
                (Numeric::Int32(a),   Numeric::Int32(b)  ) => Some(Value::Numeric(Numeric::Int32(a $op b))),
                (Numeric::Int64(a),   Numeric::Int64(b)  ) => Some(Value::Numeric(Numeric::Int64(a $op b))),
                (Numeric::Float32(a), Numeric::Float32(b)) => Some(Value::Numeric(Numeric::Float32(a $op b))),
                (Numeric::Float64(a), Numeric::Float64(b)) => Some(Value::Numeric(Numeric::Float64(a $op b))),
                (Numeric::USize(a),   Numeric::USize(b)  ) => Some(Value::Numeric(Numeric::USize(a $op b))),
                (Numeric::ISize(a),   Numeric::ISize(b)  ) => Some(Value::Numeric(Numeric::ISize(a $op b))),
                _ => None
            }
        }
    }
}


macro_rules! impl_cmp {
    ($name:ident, $op:tt) => {
        pub fn $name(&self, rhs: &Self) -> Option<Value> {
            match (self, rhs) {
                (Numeric::UInt8(a),   Numeric::UInt8(b)  ) => Some(Value::Bool(a $op b)),
                (Numeric::UInt16(a),  Numeric::UInt16(b) ) => Some(Value::Bool(a $op b)),
                (Numeric::UInt32(a),  Numeric::UInt32(b) ) => Some(Value::Bool(a $op b)),
                (Numeric::UInt64(a),  Numeric::UInt64(b) ) => Some(Value::Bool(a $op b)),
                (Numeric::UInt128(a), Numeric::UInt128(b)) => Some(Value::Bool(a $op b)),
                (Numeric::Int8(a),    Numeric::Int8(b)   ) => Some(Value::Bool(a $op b)),
                (Numeric::Int16(a),   Numeric::Int16(b)  ) => Some(Value::Bool(a $op b)),
                (Numeric::Int32(a),   Numeric::Int32(b)  ) => Some(Value::Bool(a $op b)),
                (Numeric::Int64(a),   Numeric::Int64(b)  ) => Some(Value::Bool(a $op b)),
                (Numeric::Float32(a), Numeric::Float32(b)) => Some(Value::Bool(a $op b)),
                (Numeric::Float64(a), Numeric::Float64(b)) => Some(Value::Bool(a $op b)),
                (Numeric::USize(a),   Numeric::USize(b)  ) => Some(Value::Bool(a $op b)),
                (Numeric::ISize(a),   Numeric::ISize(b)  ) => Some(Value::Bool(a $op b)),
                _ => None
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum NumericType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float32,
    Float64,
    USize,
    ISize,
}


macro_rules! cast {
    ($to:ident, $value:ident) => {
        match $to {
            NumericType::UInt8   => Numeric::UInt8($value as u8),
            NumericType::UInt16  => Numeric::UInt16($value as u16),
            NumericType::UInt32  => Numeric::UInt32($value as u32),
            NumericType::UInt64  => Numeric::UInt64($value as u64),
            NumericType::UInt128 => Numeric::UInt128($value as u128),
            NumericType::Int8    => Numeric::Int8($value as i8),
            NumericType::Int16   => Numeric::Int16($value as i16),
            NumericType::Int32   => Numeric::Int32($value as i32),
            NumericType::Int64   => Numeric::Int64($value as i64),
            NumericType::Int128  => Numeric::Int128($value as i128),
            NumericType::Float32 => Numeric::Float32($value as f32),
            NumericType::Float64 => Numeric::Float64($value as f64),
            NumericType::USize   => Numeric::USize($value as usize),
            NumericType::ISize   => Numeric::ISize($value as isize),
        }
    }
}

#[macro_export]
macro_rules! cast_to_value {
    ($from:ident, $to:tt) => {
        match $from {
            Numeric::UInt8(a)   => a as $to,
            Numeric::UInt16(a)  => a as $to,
            Numeric::UInt32(a)  => a as $to,
            Numeric::UInt64(a)  => a as $to,
            Numeric::UInt128(a) => a as $to,
            Numeric::Int8(a)    => a as $to,
            Numeric::Int16(a)   => a as $to,
            Numeric::Int32(a)   => a as $to,
            Numeric::Int64(a)   => a as $to,
            Numeric::Int128(a)  => a as $to,
            Numeric::Float32(a) => a as $to,
            Numeric::Float64(a) => a as $to,
            Numeric::USize(a)   => a as $to,
            Numeric::ISize(a)   => a as $to,
        }
    }
}


impl Numeric {
    impl_math!(add, +);
    impl_math!(sub, -);
    impl_math!(mul, *);
    impl_math!(div, /);
    impl_cmp!(greater_than, >);
    impl_cmp!(greater_than_eq, >=);
    impl_cmp!(less_than, <);
    impl_cmp!(less_than_eq, <=);
    impl_cmp!(eq, ==);

    pub fn cast(self, to: &NumericType) -> Numeric {
        match self {
            Numeric::UInt8(a)   => { return cast!(to, a); },
            Numeric::UInt16(a)  => { return cast!(to, a); },
            Numeric::UInt32(a)  => { return cast!(to, a); },
            Numeric::UInt64(a)  => { return cast!(to, a); },
            Numeric::UInt128(a) => { return cast!(to, a); },
            Numeric::Int8(a)    => { return cast!(to, a); },
            Numeric::Int16(a)   => { return cast!(to, a); },
            Numeric::Int32(a)   => { return cast!(to, a); },
            Numeric::Int64(a)   => { return cast!(to, a); },
            Numeric::Int128(a)  => { return cast!(to, a); },
            Numeric::Float32(a) => { return cast!(to, a); },
            Numeric::Float64(a) => { return cast!(to, a); },
            Numeric::USize(a)   => { return cast!(to, a); },
            Numeric::ISize(a)   => { return cast!(to, a); },
        };
    }
}
