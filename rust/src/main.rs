mod my_mod;

fn function() {
    println!("called `function()`");
}

fn main() {
    function();

    my_mod::function();

    my_mod::indirect_access();

    my_mod::nested::indirect_access();
}
