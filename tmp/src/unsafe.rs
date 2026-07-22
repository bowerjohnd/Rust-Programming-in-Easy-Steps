// examples in Chapter 12 of unsafe Rust

unsafe fn danger() { }

unsafe trait Dangerous { }

unsafe {
    total = total + static_variable;
}

// -----------NOTES from book---------------
// Unsafe can:
//      Dereference raw pointers (*i64 is a raw pointer to an i64)
//      Call unsafe functions
//      Implement unsafe traits
//      Access and modify static variables
//      Access fields of unions

untion {
    ipv4: u32;
    bytes: [4:u8];
}

// Unsafe functions can only be called from unsafe code

