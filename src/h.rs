use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::vnode::{create_vnode, deserialize_vnode, VNode, VNodeData};

#[wasm_bindgen]
pub fn h(
    tag_name: String,
    data: Option<VNodeData>,
    children: JsValue,
    text: Option<String>,
) -> VNode {
    let mut ch = None;
    // Check if children is an Array
    if let Ok(children_array) = children.dyn_into::<Array>() {
        let mut vnodes = Vec::new();

        // Iterate through the array elements and manually deserialize each element
        for i in 0..children_array.length() {
            if let Some(child) = Some(children_array.get(i)) {
                if let Some(vnode) = deserialize_vnode(&child) {
                    vnodes.push(vnode);
                }
            }
        }
        ch = Some(vnodes);
    }
    return create_vnode(tag_name, data, ch, text);
}
