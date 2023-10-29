use js_sys::{Array, Error};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, EventTarget, Node};

use crate::{html_dom_api, init::log};
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VNodeData {
    #[serde(skip)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VNode {
    sel: Option<String>,
    data: Option<VNodeData>,
    children: Option<Vec<VNode>>,
    #[serde(skip)]
    elm: Option<Node>,
    text: Option<String>,
    key: Option<String>,
}

#[wasm_bindgen]
impl VNode {
    pub fn new(
        tag: String,
        data: Option<VNodeData>,
        children: JsValue,
        text: Option<String>,
    ) -> Self {
        let mut ch: Option<Vec<VNode>> = None;

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

            // Set the manually deserialized Vec<VNode> as the children
            ch = Some(vnodes);
        }
        Self {
            sel: Some(tag),
            data: data,
            children: ch,
            elm: None,
            text: text,
            key: None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn sel(&mut self) -> Option<String> {
        let sel = self.sel.clone();
        sel
    }

    #[wasm_bindgen(getter)]
    pub fn children(&self) -> JsValue {
        // Serialize the Vec<VNode> to a JavaScript array
        let js_children = Array::new();

        if let Some(children) = &self.children {
            for vnode in children {
                let vnode_str = serialize_vnode(vnode);
                js_children.push(&vnode_str);
            }
        }

        JsValue::from(js_children)
    }

    pub fn get_text(&self) -> Option<String> {
        match self.text.clone() {
            Some(val) => return Some(val),
            None => {
                return None;
            }
        }
    }

    pub fn set_elm(&mut self, node: Node) {
        self.elm = Some(node);
    }
}

fn serialize_vnode(vnode: &VNode) -> JsValue {
    let serialized = serde_wasm_bindgen::to_value(vnode).unwrap();
    serialized
}

fn deserialize_vnode(data: &JsValue) -> Option<VNode> {
    let deserialized: VNode = serde_wasm_bindgen::from_value(data.clone()).unwrap();
    Some(deserialized)
}

#[wasm_bindgen(js_name = toVNode)]
pub fn to_vnode(node: Node) -> Result<VNode, Error> {
    let cloned_node = node.clone();
    if (!html_dom_api::is_element(cloned_node)) {
        return Err(Error::new("Provided node is not an element type"));
    } else {
        let n = node.node_name();
        return Ok(VNode::new(n.clone(), None, JsValue::NULL, None));
    }
}
