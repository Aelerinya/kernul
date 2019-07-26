use cc::Build;

fn main() {
    Build::new().file("boot.s").flag("-m32").compile("boot");
}
