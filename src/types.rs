use std::fmt::Formatter;

pub enum Atomicity {
    Strict,
    Relaxed,
}

impl std::fmt::Display for Atomicity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atomicity::Strict => write!(f, "strict"),
            Atomicity::Relaxed => write!(f, "relaxed"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Schema {
    Boolean,
    Byte,
    Short,
    Char,
    Int,
    Float,
    Symbol,
    String,
    Long,
    Date,
    Timestamp,
    Double,
    Binary,
    Long256,
}

impl std::fmt::Display for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Schema::Boolean => write!(f, "BOOLEAN"),
            Schema::Byte => write!(f, "BYTE"),
            Schema::Short => write!(f, "SHORT"),
            Schema::Char => write!(f, "CHAR"),
            Schema::Int => write!(f, "INT"),
            Schema::Float => write!(f, "FLOAT"),
            Schema::Symbol => write!(f, "SYMBOL"),
            Schema::String => write!(f, "STRING"),
            Schema::Long => write!(f, "LONG"),
            Schema::Date => write!(f, "DATE"),
            Schema::Timestamp => write!(f, "TIMESTAMP"),
            Schema::Double => write!(f, "DOUBLE"),
            Schema::Binary => write!(f, "BINARY"),
            Schema::Long256 => write!(f, "LONG256"),
        }
    }
}
