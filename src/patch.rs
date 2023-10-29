use js_sys::Error;
use wasm_bindgen::prelude::*;
use web_sys::{
    Comment, Document, DocumentFragment, Element, ElementCreationOptions, HtmlElement, Node, Text,
};

use crate::{
    html_dom_api::{append_child, create_element, create_text_node, is_element},
    vnode::VNode,
};

#[wasm_bindgen(js_name = patch)]
pub fn patch(mut old_vnode: VNode) -> Result<VNode, Error> {
    let el = create_element(old_vnode.sel().unwrap(), None).unwrap();
    let el_clone = el.clone();
    let el_clone_2 = el.clone();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let doc_node = document.query_selector("body").unwrap().unwrap();
    append_child(doc_node.into(), el.into()).unwrap();
    match old_vnode.get_text() {
        Some(val) => {
            let text_node = create_text_node(val);
            append_child(el_clone.into(), text_node.into()).unwrap();
        }
        None => {}
    }

    old_vnode.set_elm(el_clone_2.into());
    let a: VNode = old_vnode.into();
    Ok(a)
}
