use crate::checking::{scope::ScopedType};

pub const TYPE_INT: ScopedType = ScopedType {
    name: "Int",
    id: 0,
};
pub const TYPE_FLOAT: ScopedType = ScopedType {
    name: "Float",
    id: 1,
};
pub const TYPE_BOOL: ScopedType = ScopedType {
    name: "Bool",
    id: 2,
};
pub const TYPE_UNIT: ScopedType = ScopedType {
    name: "Unit",
    id: 3,
};
