mod module {
    pub mod public_module {
        // The submodule is public
        pub fn public_function() {
            println!("Hello, world!");
        } // The function is public

        fn private_function() {} // The function is private
    }

    mod private_module {}
}

use module::public_module::public_function;

fn main() {
    public_function();
    module::public_module::public_function();
    module::public_module::private_function(); // ERROR (internal_fn is private by default)
}
