#![allow(dead_code, unused_assignments, unused_imports)]

use wasm_bindgen::prelude::*;
use web_sys::{
    Comment, Document, DocumentFragment, Element, ElementCreationOptions, HtmlElement, Node, Text,
};

use crate::{dom_api::DomAPI, init::log};

pub struct HTMLDomAPI {}

impl HTMLDomAPI {
    pub fn new() -> Self {
        HTMLDomAPI {}
    }
}

impl DomAPI for HTMLDomAPI {
    fn create_element(
        &self,
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

    fn create_element_ns(
        &self,
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

    fn get_body(&self) -> Option<HtmlElement> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        return document.body();
    }

    fn insert_before(
        &self,
        parent_node: Node,
        new_node: Node,
        reference_node: Option<Node>,
    ) -> Result<Node, JsValue> {
        return parent_node.insert_before(&new_node, reference_node.as_ref());
    }

    fn remove_child(&self, node: Node, child: Node) -> Result<Node, JsValue> {
        return node.remove_child(&child);
    }

    fn append_child(&self, node: Node, child: Node) -> Result<Node, JsValue> {
        return node.append_child(&child);
    }

    fn parent_node(&self, node: Node) -> Option<Node> {
        return node.parent_node();
    }

    fn next_sibling(&self, node: Node) -> Option<Node> {
        return node.next_sibling();
    }

    fn create_document_fragment(&self) -> Result<DocumentFragment, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        return Ok(document.create_document_fragment());
    }

    fn create_text_node(&self, text: String) -> Text {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        return document.create_text_node(&text);
    }

    fn create_comment(&self, text: String) -> Comment {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        return document.create_comment(&text);
    }

    fn is_document_fragment(&self, node: Node) -> bool {
        return node.node_type() == 11;
    }

    fn tag_name(&self, elm: Element) -> String {
        return elm.tag_name();
    }

    fn set_text_content(&self, node: Node, text: Option<String>) {
        match text {
            Some(val) => node.set_node_value(Some(&val)),
            None => {}
        }
    }

    fn get_text_content(&self, node: Node) -> Option<String> {
        return node.text_content();
    }

    fn is_text(&self, node: Node) -> bool {
        return node.node_type() == 3;
    }

    fn is_comment(&self, node: Node) -> bool {
        return node.node_type() == 8;
    }

    fn is_element(&self, node: Node) -> bool {
        return self.is_element_from_node_ref(&node);
    }

    fn is_element_from_node_ref(&self, node: &Node) -> bool {
        return node.node_type() == Node::ELEMENT_NODE;
    }

    fn query_element(&self, query: String) -> Result<Option<Element>, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        return document.query_selector(query.as_str());
    }
}
