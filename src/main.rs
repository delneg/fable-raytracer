#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#[path = "./RayTracer.rs"]
pub(crate) mod import_ec9511b6;
pub use import_ec9511b6::*;
use fable_library_rust::*;
pub mod TestApp {
    use super::*;
    pub fn main(_args: &Rc<MutCell<Vec<Rc<str>>>>) -> i32 {
        let patternInput: (i32, i32, i32, i32) =
            (0i32, 0i32, 1024i32, 1024i32);
        let w: i32 = patternInput.2.clone();
        let h: i32 = patternInput.3.clone();
        let data: Rc<MutCell<Vec<u8>>> =
            Native::arrayCreate(&(w * h * 4i32), &0u8);
        println!("{0}", &String_::string(&"Raytracer running..."));
        RayTracerDemo::renderScene(&data, &patternInput.0.clone(),
                                   &patternInput.1.clone(), &w, &h, &0.0f64);
        println!("Ray tracing done:\n - rendered image size: ({0}x{1})\n", &w,
                 &h);
        0i32
    }
}
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<Rc<str>> = args[1..].iter().map(|s| String_::string(s)).collect();
    TestApp::main(&Native::arrayFrom(&args));
}
