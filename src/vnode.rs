use js_sys::{Array, Error};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, EventTarget, Node};

use crate::{
    dom_api::DomAPI,
    html_dom_api::{self, HTMLDomAPI},
    init::log,
};
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
    #[wasm_bindgen(getter)]
    pub fn sel(&mut self) -> Option<String> {
        let sel = self.sel.clone();
        sel
    }

    #[wasm_bindgen(getter)]
    pub fn elm(&mut self) -> Option<Node> {
        return self.elm.clone();
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

    pub fn get_data(&self) -> Option<VNodeData> {
        return self.data.clone();
    }

    pub fn set_elm(&mut self, node: Node) {
        self.elm = Some(node);
    }
}

pub fn create_vnode(
    tag: String,
    data: Option<VNodeData>,
    children: Option<Vec<VNode>>,
    text: Option<String>,
) -> VNode {
    VNode {
        sel: Some(tag),
        data: data,
        children: children,
        elm: None,
        text: text,
        key: None,
    }
}

#[wasm_bindgen(js_name = toVNode)]
pub fn to_vnode(node: Node) -> Result<VNode, Error> {
    let dom_api = HTMLDomAPI::new();
    if !dom_api.is_element_from_node_ref(&node) {
        return Err(Error::new("Provided node is not an element type"));
    } else {
        let n = node.node_name();
        return Ok(create_vnode(n.clone(), None, None, None));
    }
}

pub fn serialize_vnode(vnode: &VNode) -> JsValue {
    let serialized = serde_wasm_bindgen::to_value(vnode).unwrap();
    serialized
}

pub fn deserialize_vnode(data: &JsValue) -> Option<VNode> {
    let deserialized: VNode = serde_wasm_bindgen::from_value(data.clone()).unwrap();
    Some(deserialized)
}
