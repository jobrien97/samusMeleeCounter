#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod samus;

#[skyline::main(name = "samusMeleeCounter")]
pub fn main() {
    samus::install();
}
