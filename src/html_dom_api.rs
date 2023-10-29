#![allow(dead_code, unused_assignments, unused_imports)]

use wasm_bindgen::prelude::*;
use web_sys::{
    Comment, Document, DocumentFragment, Element, ElementCreationOptions, HtmlElement, Node, Text,
};

use crate::init::log;

pub fn create_element(
    tag_name: String,
    options: Option<ElementCreationOptions>,
) -> Result<Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    match options {
        Some(val) => {
            return document.create_element_with_element_creation_options(&tag_name, &val);
        }
        None => {
            return document.create_element(&tag_name);
        }
    }
}

pub fn create_element_ns(
    name_space_uri: String,
    qualified_name: String,
    options: Option<ElementCreationOptions>,
) -> Result<Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    match options {
        Some(val) => {
            return document.create_element_ns_with_element_creation_options(
                Some(&name_space_uri),
                &qualified_name,
                &val,
            );
        }
        None => {
            return document.create_element_ns(Some(&name_space_uri), &qualified_name);
        }
    }
}

pub fn insert_before(
    parent_node: Node,
    new_node: Node,
    reference_node: Option<Node>,
) -> Result<Node, JsValue> {
    return parent_node.insert_before(&new_node, reference_node.as_ref());
}

pub fn remove_child(node: Node, child: Node) -> Result<Node, JsValue> {
    return node.remove_child(&child);
}

pub fn append_child(node: Node, child: Node) -> Result<Node, JsValue> {
    return node.append_child(&child);
}

pub fn parent_node(node: Node) -> Option<Node> {
    return node.parent_node();
}

pub fn next_sibling(node: Node) -> Option<Node> {
    return node.next_sibling();
}

pub fn create_document_fragment() -> Result<DocumentFragment, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    return Ok(document.create_document_fragment());
}

pub fn create_text_node(text: String) -> Text {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    return document.create_text_node(&text);
}

pub fn create_comment(text: String) -> Comment {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    return document.create_comment(&text);
}

pub fn is_document_fragment(node: Node) -> bool {
    return node.node_type() == 11;
}

pub fn tag_name(elm: Element) -> String {
    return elm.tag_name();
}

pub fn set_text_content(node: Node, text: Option<String>) {
    match text {
        Some(val) => node.set_node_value(Some(&val)),
        None => {}
    }
}

pub fn get_text_content(node: Node) -> Option<String> {
    return node.text_content();
}

pub fn is_text(node: Node) -> bool {
    return node.node_type() == 3;
}

pub fn is_comment(node: Node) -> bool {
    return node.node_type() == 8;
}

pub fn is_element(node: Node) -> bool {
    return node.node_type() == Node::ELEMENT_NODE;
}
