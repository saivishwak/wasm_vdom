#![allow(dead_code, unused_variables)]

use js_sys::Error;
use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut()
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure:closure, token }
    }
}


// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        cancelInterval(self.token);
    }
}


#[wasm_bindgen(js_name = test)]
pub fn test(_name: &str) -> Result<Interval, JsValue> {
    log("Testing");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    Ok(Interval::new(1_000, move || {
       let mut rng = rand::thread_rng();
       let n1: u8 = rng.gen(); 
       log(&format!("{}", n1));
        val.set_text_content(Some(&format!("{}",n1)))
    }))
}

// #[wasm_bindgen]
// pub struct VNode{
//     tag:String
// }


// #[wasm_bindgen]
// impl VNode {
//     #[wasm_bindgen(getter)]
//     pub fn tag(&self) -> String {
//         self.tag.clone()
//     }
// }

// impl From<JsValue> for VNode {
//     fn from(js_value: JsValue) -> Self {
//         // Here, you should write code to convert a JsValue into a VNode
//         // This might involve accessing and extracting data from the JsValue
//         // and constructing a VNode.
//         // For simplicity, we'll assume a VNode can be constructed from a string.

//         if let Some(js_string) = js_value.as_string() {
//             VNode {
//                 tag: js_string
//             }
//         } else {
//             // Handle the case where conversion is not possible.
//             panic!("Conversion from JsValue to VNode failed.");
//         }
//     }
// }


#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
       
    }
}
