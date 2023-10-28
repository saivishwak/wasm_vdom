use js_sys::Error;
use wasm_bindgen::prelude::*;

use crate::vnode::VNode;

#[wasm_bindgen(js_name = patch)]
pub fn patch(old_vnode: VNode, new_vnode: VNode) -> Result<VNode, Error> {
    let a: VNode = old_vnode.into();
    Ok(a)
}
