use js_sys::Error;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};

use crate::{dom_api::DomAPI, html_dom_api::HTMLDomAPI, vnode::VNode};

struct PatchNode {
    el: Element,
    node: VNode,
}

fn patch_vnode(node: Node, mut vnode: VNode) -> PatchNode {
    let api = HTMLDomAPI::new();
    let el = api.create_element(vnode.sel().unwrap(), None).unwrap();
    let el_clone = el.clone();
    let el_clone_2 = el.clone();
    api.append_child(node, el.into()).unwrap();
    match vnode.get_text() {
        Some(val) => {
            let text_node = api.create_text_node(val);
            api.append_child(el_clone.into(), text_node.into()).unwrap();
        }
        None => {}
    }
    return PatchNode {
        el: el_clone_2,
        node: vnode,
    };
}

#[wasm_bindgen(js_name = patch)]
pub fn patch(mut old_vnode: VNode) -> Result<VNode, Error> {
    let api = HTMLDomAPI::new();
    let doc_node = api.get_body().unwrap();

    let mut patched_node = patch_vnode(doc_node.into(), old_vnode);

    patched_node.node.set_elm(patched_node.el.into());
    Ok(patched_node.node)
}
