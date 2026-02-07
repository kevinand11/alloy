pub enum Precedence {
    Lowest,
    Equality,   // == or !=
    Comparison, // <, <=, >, >=
    Sum,        // + or -
    Product,    // * or /
    Order,      // ^
    Group,      // { }
    Prefix,     // !X or -X
}
