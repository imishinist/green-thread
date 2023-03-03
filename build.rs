use cc::Build;

const ASM_FILE: &str = "asm/context.S";
const LIB_FILE: &str = "libcontext.a";

fn main() {
    Build::new().file(&ASM_FILE).compile(&LIB_FILE);
    println!("cargo:rerun-if-changed={}", ASM_FILE);
}
