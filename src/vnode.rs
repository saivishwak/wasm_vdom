use js_sys::Error;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, EventTarget, Node};

use crate::{html_dom_api, init::log};

#[wasm_bindgen]
pub struct VNodeData {
    props: Option<HashMap<String, JsValue>>,
    attrs: Option<i32>,
    class: Option<i32>,
    style: Option<HashMap<String, String>>,
    on: Option<Vec<i32>>,
    // hook: Option<i32>,
    key: Option<String>,
    ns: Option<String>,
}

#[wasm_bindgen]
pub struct VNode {
    sel: Option<String>,
    data: Option<VNodeData>,
    children: Option<Vec<VNode>>,
    elm: Option<Node>,
    text: Option<String>,
    key: Option<String>,
}

#[wasm_bindgen]
impl VNode {
    pub fn new(tag: String) -> Self {
        Self {
            sel: Some(tag),
            data: None,
            children: None,
            elm: None,
            text: None,
            key: None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn sel(&mut self) -> Option<String> {
        let sel = self.sel.clone();
        sel
    }
}

#[wasm_bindgen(js_name = toVNode)]
pub fn to_vnode(node: Node) -> Result<VNode, Error> {
    let cloned_node = node.clone();
    if (!html_dom_api::is_element(cloned_node)) {
        return Err(Error::new("Provided node is not an element type"));
    } else {
        let n = node.node_name();
        return Ok(VNode::new(n.clone()));
    }
}
