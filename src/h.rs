use wasm_bindgen::JsValue;

use crate::vnode::{VNode, VNodeData};

pub fn h(tag_name: String, data: Option<VNodeData>, children: Option<Vec<VNode>>) -> VNode {
    let new_vnode = VNode::new(tag_name, None, JsValue::NULL, None);

    return new_vnode;
}
