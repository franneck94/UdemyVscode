// Items in modules default to private visibility.
fn private_function() {
    println!("called `my_mod::private_function()`");
}

// Use the `pub` modifier to override default visibility.
pub fn function() -> i32 {
    println!("called `my_mod::function()`");
    return 0_i32;
}

// Items can access other items in the same module,
// even when private.
pub fn indirect_access() {
    print!("called `my_mod::indirect_access()`, that\n> ");
    private_function();
}

// Modules can also be nested
pub mod nested {
    pub fn function() {
        println!("called `my_mod::nested::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    #[allow(dead_code)]
    fn private_function() {
        println!("called `my_mod::nested::private_function()`");
    }

    // Functions declared using `pub(in path)` syntax are only visible
    // within the given path. `path` must be a parent or ancestor module
    pub(in crate::my_mod) fn public_function_in_my_mod() {
        print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
        public_function_in_nested();
    }

    // Functions declared using `pub(self)` syntax are only visible within
    // the current module, which is the same as leaving them private
    pub(self) fn public_function_in_nested() {
        println!("called `my_mod::nested::public_function_in_nested()`");
    }

    // Functions declared using `pub(super)` syntax are only visible within
    // the parent module
    pub(super) fn public_function_in_super_mod() {
        println!("called `my_mod::nested::public_function_in_super_mod()`");
    }
}

pub fn call_public_function_in_my_mod() {
    print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
    nested::public_function_in_my_mod();
    print!("> ");
    nested::public_function_in_super_mod();
}

// pub(crate) makes functions visible only within the current crate
pub(crate) fn public_function_in_crate() {
    println!("called `my_mod::public_function_in_crate()`");
}

// Nested modules follow the same rules for visibility
mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
        println!("called `my_mod::private_nested::function()`");
    }

    // Private parent items will still restrict the visibility of a child item,
    // even if it is declared as visible within a bigger scope.
    #[allow(dead_code)]
    pub(crate) fn restricted_function() {
        println!("called `my_mod::private_nested::restricted_function()`");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function1() {
        assert_eq!(function(), 0);
    }

    #[test]
    fn test_function2() {
        assert_eq!(function(), 0);
    }
}
