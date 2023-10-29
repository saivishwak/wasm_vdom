use wasm_bindgen::prelude::*;
use web_sys::{
    Comment, DocumentFragment, Element, ElementCreationOptions, HtmlElement, Node, Text,
};

use crate::html_dom_api::HTMLDomAPI;

pub trait DomAPI {
    fn create_element(
        &self,
        tag_name: String,
        options: Option<ElementCreationOptions>,
    ) -> Result<Element, JsValue>;
    fn create_element_ns(
        &self,
        name_space_uri: String,
        qualified_name: String,
        options: Option<ElementCreationOptions>,
    ) -> Result<Element, JsValue>;

    fn insert_before(
        &self,
        parent_node: Node,
        new_node: Node,
        reference_node: Option<Node>,
    ) -> Result<Node, JsValue>;

    fn remove_child(&self, node: Node, child: Node) -> Result<Node, JsValue>;

    fn append_child(&self, node: Node, child: Node) -> Result<Node, JsValue>;

    fn parent_node(&self, node: Node) -> Option<Node>;

    fn next_sibling(&self, node: Node) -> Option<Node>;

    fn create_document_fragment(&self) -> Result<DocumentFragment, JsValue>;

    fn create_text_node(&self, text: String) -> Text;

    fn create_comment(&self, text: String) -> Comment;

    fn is_document_fragment(&self, node: Node) -> bool;

    fn tag_name(&self, elm: Element) -> String;

    fn set_text_content(&self, node: Node, text: Option<String>);

    fn get_text_content(&self, node: Node) -> Option<String>;

    fn get_body(&self) -> Option<HtmlElement>;

    fn is_text(&self, node: Node) -> bool;

    fn is_comment(&self, node: Node) -> bool;

    fn is_element(&self, node: Node) -> bool;

    fn is_element_from_node_ref(&self, node: &Node) -> bool;

    fn query_element(&self, query: String) -> Result<Option<Element>, JsValue>;
}
