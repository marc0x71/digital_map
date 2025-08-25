#![allow(unused)]

use crate::tokenizer::Token;
use std::{fmt::Debug, ops::Deref};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum NodeType {
    Root,
    Exact(u8),
    Repeatable(u8),
}

#[derive(Debug)]
pub struct Node<T> {
    children: Vec<Box<Node<T>>>,
    value: Option<T>,
    node_type: NodeType,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn root() -> Self {
        Self {
            children: vec![],
            value: None,
            node_type: NodeType::Root,
        }
    }

    fn new(node_type: NodeType) -> Self {
        Self {
            children: vec![],
            value: None,
            node_type,
        }
    }

    pub fn add_with(&mut self, digit: u8, node_type: NodeType) -> &mut Node<T> {
        match self.node_type {
            NodeType::Exact(_) | NodeType::Root => self.add_if_missing(digit, node_type),
            NodeType::Repeatable(my_digit) => {
                if my_digit == digit {
                    self
                } else {
                    self.add_if_missing(digit, node_type)
                }
            }
        }
    }

    fn merge(&mut self, other_node_type: &NodeType) {
        if let (NodeType::Exact(_), NodeType::Repeatable(_)) = (&self.node_type, other_node_type) {
            self.node_type = other_node_type.clone()
        }
    }

    fn add_if_missing(&mut self, digit: u8, node_type: NodeType) -> &mut Node<T> {
        if let Some(index) = self.can_handle_index(digit) {
            self.children[index].merge(&node_type);
            return &mut self.children[index];
        }

        let child = Self::new(node_type);
        self.children.push(Box::new(child));

        self.children.last_mut().unwrap()
    }

    fn can_handle_index(&self, digit: u8) -> Option<usize> {
        self.children
            .iter()
            .position(|child| child.can_handle(digit))
    }

    fn can_handle(&self, digit: u8) -> bool {
        match self.node_type {
            NodeType::Root => false,
            NodeType::Exact(my_digit) => my_digit == digit,
            NodeType::Repeatable(my_digit) => {
                my_digit == digit || self.can_handle_index(digit).is_some()
            }
        }
    }

    pub fn get(&self, digit: u8) -> Option<&Node<T>> {
        println!("]] Node.get({digit}) {:?}", self.node_type);
        if let Some(index) = self.can_handle_index(digit) {
            return Some(&self.children[index]);
        }
        match self.node_type {
            NodeType::Repeatable(my_digit) if my_digit == digit => Some(self),
            _ => None,
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn dump(&self, level: u32) {
        if level == 0 {
            println!("----------");
        }
        let spaces = " ".repeat(level as usize);
        println!("{spaces} | {:?} --> {:?}", self.node_type, self.value);

        for child in &self.children {
            child.dump(level + 2);
        }
        if level == 0 {
            println!("----------");
        }
    }
}

impl From<&Token> for NodeType {
    fn from(value: &Token) -> Self {
        match value.kind {
            crate::tokenizer::TokenKind::Single => NodeType::Exact(value.digit),
            crate::tokenizer::TokenKind::AtLeastOne => NodeType::Repeatable(value.digit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::{Token, TokenKind};

    // TEST CREAZIONE E INIZIALIZZAZIONE
    #[test]
    fn test_node_root_creation() {
        let node: Node<String> = Node::root();

        assert_eq!(node.node_type, NodeType::Root);
        assert!(node.children.is_empty());
        assert!(node.value.is_none());
    }

    #[test]
    fn test_node_new_exact() {
        let node: Node<i32> = Node::new(NodeType::Exact(5));

        assert_eq!(node.node_type, NodeType::Exact(5));
        assert!(node.children.is_empty());
        assert!(node.value.is_none());
    }

    #[test]
    fn test_node_new_repeatable() {
        let node: Node<String> = Node::new(NodeType::Repeatable(3));

        assert_eq!(node.node_type, NodeType::Repeatable(3));
        assert!(node.children.is_empty());
        assert!(node.value.is_none());
    }

    // TEST SET/GET VALUE
    #[test]
    fn test_set_and_get_value() {
        let mut node: Node<String> = Node::root();

        assert_eq!(node.get_value(), None);

        node.set_value("test_value".to_string());
        assert_eq!(node.get_value(), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_overwrite_value() {
        let mut node: Node<i32> = Node::root();

        node.set_value(42);
        assert_eq!(node.get_value(), Some(&42));

        node.set_value(99);
        assert_eq!(node.get_value(), Some(&99));
    }

    // TEST CAN_HANDLE
    #[test]
    fn test_root_can_handle() {
        let node: Node<String> = Node::root();

        // Root non può gestire nessun digit direttamente
        assert!(!node.can_handle(0));
        assert!(!node.can_handle(5));
        assert!(!node.can_handle(9));
    }

    #[test]
    fn test_exact_can_handle() {
        let node: Node<String> = Node::new(NodeType::Exact(7));

        assert!(node.can_handle(7));
        assert!(!node.can_handle(6));
        assert!(!node.can_handle(8));
        assert!(!node.can_handle(0));
    }

    #[test]
    fn test_repeatable_can_handle() {
        let node: Node<String> = Node::new(NodeType::Repeatable(4));

        assert!(node.can_handle(4));
        assert!(!node.can_handle(3));
        assert!(!node.can_handle(5));
    }

    #[test]
    fn test_repeatable_can_handle_with_children() {
        let mut node: Node<String> = Node::new(NodeType::Repeatable(3));

        // Aggiungi un figlio
        node.add_with(5, NodeType::Exact(5));

        // Ora può gestire sia il suo digit che quello del figlio
        assert!(node.can_handle(3)); // Suo digit
        assert!(node.can_handle(5)); // Digit del figlio
        assert!(!node.can_handle(7)); // Digit non gestito
    }

    // TEST ADD_WITH - ROOT
    #[test]
    fn test_root_add_with() {
        let mut root: Node<String> = Node::root();

        let child = root.add_with(3, NodeType::Exact(3));

        assert_eq!(child.node_type, NodeType::Exact(3));
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn test_root_add_multiple_children() {
        let mut root: Node<String> = Node::root();

        root.add_with(1, NodeType::Exact(1));
        root.add_with(5, NodeType::Exact(5));
        root.add_with(9, NodeType::Exact(9));

        assert_eq!(root.children.len(), 3);

        // Verifica che i figli abbiano i tipi corretti
        assert!(
            root.children
                .iter()
                .any(|child| child.node_type == NodeType::Exact(1))
        );
        assert!(
            root.children
                .iter()
                .any(|child| child.node_type == NodeType::Exact(5))
        );
        assert!(
            root.children
                .iter()
                .any(|child| child.node_type == NodeType::Exact(9))
        );
    }

    // TEST ADD_WITH - EXACT
    #[test]
    fn test_exact_add_with() {
        let mut node: Node<String> = Node::new(NodeType::Exact(2));

        let child = node.add_with(7, NodeType::Exact(7));

        assert_eq!(child.node_type, NodeType::Exact(7));
        assert_eq!(node.children.len(), 1);
    }

    // TEST ADD_WITH - REPEATABLE
    #[test]
    fn test_repeatable_add_with_same_digit() {
        let mut node: Node<String> = Node::new(NodeType::Repeatable(5));

        // Quando aggiungi lo stesso digit, dovrebbe restituire se stesso
        let same_node = node.add_with(5, NodeType::Exact(5));

        assert_eq!(same_node.node_type, NodeType::Repeatable(5));
        assert_eq!(same_node as *const _, &node as *const _); // Stesso puntatore
        assert!(node.children.is_empty()); // Non dovrebbe creare figli
    }

    #[test]
    fn test_repeatable_add_with_different_digit() {
        let mut node: Node<String> = Node::new(NodeType::Repeatable(3));

        let child = node.add_with(8, NodeType::Exact(8));

        assert_eq!(child.node_type, NodeType::Exact(8));
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.node_type, NodeType::Repeatable(3)); // Nodo originale non cambiato
    }

    // TEST GET
    #[test]
    fn test_root_get() {
        let mut root: Node<String> = Node::root();

        // Root senza figli non dovrebbe trovare nulla
        assert!(root.get(5).is_none());

        // Aggiungi un figlio
        root.add_with(5, NodeType::Exact(5));

        // Ora dovrebbe trovarlo
        assert!(root.get(5).is_some());
        assert_eq!(root.get(5).unwrap().node_type, NodeType::Exact(5));

        // Altri digit non dovrebbero essere trovati
        assert!(root.get(3).is_none());
    }

    #[test]
    fn test_exact_get() {
        let mut node: Node<String> = Node::new(NodeType::Exact(4));

        // Exact non può restituire se stesso per il proprio digit
        assert!(node.get(4).is_none());

        // Ma può avere figli
        node.add_with(7, NodeType::Exact(7));
        assert!(node.get(7).is_some());
        assert!(node.get(8).is_none());
    }

    #[test]
    fn test_repeatable_get_same_digit() {
        let mut node: Node<String> = Node::new(NodeType::Repeatable(6));

        // Repeatable dovrebbe restituire se stesso per il proprio digit
        let same_node = node.get(6);
        assert!(same_node.is_some());
        assert_eq!(same_node.unwrap().node_type, NodeType::Repeatable(6));
        assert_eq!(same_node.unwrap() as *const _, &node as *const _);
    }

    #[test]
    fn test_repeatable_add_with_same_digit_no_child_created() {
        let mut node: Node<String> = Node::new(NodeType::Repeatable(3));

        let result = node.add_with(3, NodeType::Exact(3));

        // Dovrebbe restituire se stesso, non creare figli
        assert_eq!(result as *const _, &node as *const _);
        assert!(node.children.is_empty());
        assert_eq!(node.get(3).unwrap() as *const _, &node as *const _);
    }

    // TEST MERGE
    #[test]
    fn test_merge_exact_to_repeatable() {
        let mut node: Node<String> = Node::new(NodeType::Exact(5));

        node.merge(&NodeType::Repeatable(5));

        assert_eq!(node.node_type, NodeType::Repeatable(5));
    }

    #[test]
    fn test_merge_no_change_same_type() {
        let mut exact_node: Node<String> = Node::new(NodeType::Exact(3));
        let original_type = exact_node.node_type.clone();

        exact_node.merge(&NodeType::Exact(3));

        assert_eq!(exact_node.node_type, original_type);
    }

    #[test]
    fn test_merge_no_change_root() {
        let mut root: Node<String> = Node::root();

        root.merge(&NodeType::Exact(7));

        assert_eq!(root.node_type, NodeType::Root);
    }

    // TEST ADD_IF_MISSING CON MERGE
    #[test]
    fn test_add_if_missing_creates_new_node() {
        let mut root: Node<String> = Node::root();

        let child = root.add_if_missing(4, NodeType::Exact(4));

        assert_eq!(child.node_type, NodeType::Exact(4));
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn test_add_if_missing_finds_existing_and_merges() {
        let mut root: Node<String> = Node::root();

        // Crea prima un nodo Exact
        root.add_if_missing(6, NodeType::Exact(6));
        assert_eq!(root.children[0].node_type, NodeType::Exact(6));

        // Ora prova ad aggiungere lo stesso digit come Repeatable
        let child = root.add_if_missing(6, NodeType::Repeatable(6));

        // Dovrebbe restituire lo stesso nodo ma con merge applicato
        assert_eq!(child.node_type, NodeType::Repeatable(6));
        assert_eq!(root.children.len(), 1); // Non dovrebbe creare un nuovo figlio
    }

    // TEST FROM TRAIT
    #[test]
    fn test_from_token_single() {
        let token = Token {
            digit: 7,
            kind: TokenKind::Single,
        };

        let node_type: NodeType = (&token).into();

        assert_eq!(node_type, NodeType::Exact(7));
    }

    #[test]
    fn test_from_token_at_least_one() {
        let token = Token {
            digit: 9,
            kind: TokenKind::AtLeastOne,
        };

        let node_type: NodeType = (&token).into();

        assert_eq!(node_type, NodeType::Repeatable(9));
    }

    // TEST SCENARI COMPLESSI
    #[test]
    fn test_complex_tree_building() {
        let mut root: Node<String> = Node::root();

        // Costruisci: Root -> Exact(1) -> Exact(2) -> Repeatable(3) -> Exact(4)
        let node1 = root.add_with(1, NodeType::Exact(1));
        let node2 = node1.add_with(2, NodeType::Exact(2));
        let node3 = node2.add_with(3, NodeType::Repeatable(3));
        let node4 = node3.add_with(4, NodeType::Exact(4));

        // Imposta un valore nel nodo finale
        node4.set_value("final_value".to_string());

        // Verifica la navigazione
        let found1 = root.get(1).unwrap();
        let found2 = found1.get(2).unwrap();
        let found3 = found2.get(3).unwrap(); // Dovrebbe restituire se stesso (Repeatable)
        let found4 = found3.get(4).unwrap();

        assert_eq!(found4.get_value(), Some(&"final_value".to_string()));
    }

    #[test]
    fn test_repeatable_with_bypass() {
        let mut root: Node<String> = Node::root();

        // Simula pattern "12[3]*4"
        let node1 = root.add_with(1, NodeType::Exact(1));
        let node2 = node1.add_with(2, NodeType::Exact(2));

        // Percorso 1: diretto a 4 (bypass del 3)
        let node4_direct = node2.add_with(4, NodeType::Exact(4));
        node4_direct.set_value("bypassed".to_string());

        // Percorso 2: attraverso 3 ripetibile
        let node3_rep = node2.add_with(3, NodeType::Repeatable(3));
        let node4_via_3 = node3_rep.add_with(4, NodeType::Exact(4));
        node4_via_3.set_value("via_repeat".to_string());

        // Test navigazione diretta: 1-2-4
        let path_direct = root.get(1).unwrap().get(2).unwrap().get(4).unwrap();
        assert_eq!(path_direct.get_value(), Some(&"bypassed".to_string()));

        // Test navigazione con ripetizione: 1-2-3-4
        let path_via_repeat = root
            .get(1)
            .unwrap()
            .get(2)
            .unwrap()
            .get(3)
            .unwrap()
            .get(4)
            .unwrap();
        assert_eq!(path_via_repeat.get_value(), Some(&"via_repeat".to_string()));

        // Test ripetizione: 1-2-3-3-4
        let path_repeat_twice = root
            .get(1)
            .unwrap()
            .get(2)
            .unwrap()
            .get(3)
            .unwrap()
            .get(3)
            .unwrap()
            .get(4)
            .unwrap();
        assert_eq!(
            path_repeat_twice.get_value(),
            Some(&"via_repeat".to_string())
        );
    }

    // TEST EDGE CASES
    #[test]
    fn test_can_handle_index_empty_children() {
        let node: Node<String> = Node::root();
        assert_eq!(node.can_handle_index(5), None);
    }

    #[test]
    fn test_can_handle_index_found() {
        let mut root: Node<String> = Node::root();

        root.add_with(3, NodeType::Exact(3));
        root.add_with(7, NodeType::Exact(7));
        root.add_with(1, NodeType::Exact(1));

        assert_eq!(root.can_handle_index(3), Some(0));
        assert_eq!(root.can_handle_index(7), Some(1));
        assert_eq!(root.can_handle_index(1), Some(2));
        assert_eq!(root.can_handle_index(9), None);
    }

    #[test]
    fn test_dump_does_not_panic() {
        let mut root: Node<String> = Node::root();

        root.add_with(1, NodeType::Exact(1))
            .set_value("test".to_string());

        // Test che dump() non causi panic
        root.dump(0);
        root.dump(5); // Con indentazione diversa
    }

    // TEST TIPI DIVERSI
    #[test]
    fn test_different_value_types() {
        let mut int_node: Node<i32> = Node::root();
        int_node.set_value(42);
        assert_eq!(int_node.get_value(), Some(&42));

        let mut bool_node: Node<bool> = Node::root();
        bool_node.set_value(true);
        assert_eq!(bool_node.get_value(), Some(&true));

        let mut vec_node: Node<Vec<String>> = Node::root();
        vec_node.set_value(vec!["hello".to_string()]);
        assert_eq!(vec_node.get_value(), Some(&vec!["hello".to_string()]));
    }
}
