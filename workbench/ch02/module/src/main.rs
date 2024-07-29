mod hoge;
mod module_a;
mod module_hello;
use module_a::module_b;

fn main() {
    module_hello::print_hello();
    hoge::fuga::fuga();
    hoge::hoge();
    module_a::func_a();
    module_a::module_b::func_b();
    module_b::func_b();
}
