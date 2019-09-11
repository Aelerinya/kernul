use cc::Build;

fn main() {
    Build::new().file("boot.s").flag("-m32").compile("boot");
    Build::new().file("gdt.s").flag("-m32").compile("gdt");
    Build::new().file("idt.s").flag("-m32").compile("idt");
}
