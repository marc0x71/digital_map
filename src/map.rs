#![allow(unused)]

use std::{iter, ops::Deref, thread::current};

fn schema<T>(v: &[Option<T>]) -> String {
    let s: String = v
        .iter()
        .map(|c| c.as_ref().map(|_| 'X').unwrap_or('.'))
        .collect::<Vec<char>>()
        .into_iter()
        .collect();

    s
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Node<T> {
    digits: [Option<Box<Node<T>>>; 10],
    value: Option<T>,
}

impl<T> Node<T>
where
    T: Default,
{
    fn add(&mut self, digit: usize) -> &mut Node<T> {
        self.digits[digit]
            .get_or_insert(Box::new(Self::default()))
            .as_mut()
    }

    fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    fn get_value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    fn get(&self, digit: usize) -> Option<&Node<T>> {
        self.digits[digit].as_deref()
    }

    fn schema(&self) -> String {
        schema(&self.digits)
    }
}

impl<T> Default for Node<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            digits: Default::default(),
            value: None,
        }
    }
}

#[derive(Debug)]
struct Map<T> {
    root: Box<Node<T>>,
}

impl<T> Map<T>
where
    T: Default,
{
    fn add(&mut self, input: &str, value: T) {
        let mut current = self.root.as_mut();
        for c in input.chars() {
            let digit = c.to_digit(10).expect("input must contain only digits") as usize;
            current = current.add(digit);
        }
        current.set_value(value);
    }

    fn get(&self, input: &str) -> Option<&T> {
        let mut current = self.root.as_ref();
        for c in input.chars() {
            // println!("{} {}", c, current.schema());
            let digit = c.to_digit(10).expect("input must contain only digits") as usize;
            current = current.get(digit)?;
        }
        current.get_value()
    }
}

impl<T> Default for Map<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            root: Box::new(Node::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_default() {
        let node: Node<i32> = Node::default();
        assert_eq!(node.value, None);
        // Verifica che tutti i digit siano inizialmente None
        for digit in &node.digits {
            assert!(digit.is_none());
        }
    }

    #[test]
    fn test_node_add_single_digit() {
        let mut node: Node<String> = Node::default();

        // Prima verifica che il digit non esista
        assert!(node.get(5).is_none());

        // Aggiungi il digit e verifica che il nodo figlio sia stato creato
        {
            let child = node.add(5);
            assert!(child.value.is_none());
        } // Il borrow mutabile termina qui

        // Ora possiamo verificare che il digit sia stato creato
        assert!(node.get(5).is_some());

        // Verifica che gli altri digit siano ancora None
        for i in 0..10 {
            if i != 5 {
                assert!(node.get(i).is_none());
            }
        }
    }

    #[test]
    fn test_node_set_and_get_value() {
        let mut node: Node<String> = Node::default();

        // Inizialmente non c'è valore
        assert_eq!(node.get_value(), None);

        // Imposta un valore
        node.set_value("test_value".to_string());
        assert_eq!(node.get_value(), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_node_get_existing_digit() {
        let mut node: Node<i32> = Node::default();
        node.add(3);

        let child = node.get(3);
        assert!(child.is_some());

        let non_existing = node.get(7);
        assert!(non_existing.is_none());
    }

    #[test]
    fn test_map_default() {
        let map: Map<String> = Map::default();
        // Il root dovrebbe esistere
        assert!(map.root.value.is_none());
    }

    #[test]
    fn test_map_add_and_get_single_digit() {
        let mut map: Map<String> = Map::default();

        map.add("5", "five".to_string());

        let result = map.get("5");
        assert_eq!(result, Some(&"five".to_string()));
    }

    #[test]
    fn test_map_add_and_get_multiple_digits() {
        let mut map: Map<String> = Map::default();

        map.add("123", "one-two-three".to_string());

        let result = map.get("123");
        assert_eq!(result, Some(&"one-two-three".to_string()));
    }

    #[test]
    fn test_map_get_non_existing() {
        let mut map: Map<String> = Map::default();

        map.add("123", "exists".to_string());

        // Testa chiavi che non esistono
        assert_eq!(map.get("124"), None);
        assert_eq!(map.get("12"), None);
        assert_eq!(map.get("1234"), None);
        assert_eq!(map.get("456"), None);
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map: Map<i32> = Map::default();

        map.add("1", 100);
        map.add("12", 200);
        map.add("123", 300);
        map.add("2", 400);
        map.add("21", 500);

        assert_eq!(map.get("1"), Some(&100));
        assert_eq!(map.get("12"), Some(&200));
        assert_eq!(map.get("123"), Some(&300));
        assert_eq!(map.get("2"), Some(&400));
        assert_eq!(map.get("21"), Some(&500));
    }

    #[test]
    fn test_map_overlapping_keys() {
        let mut map: Map<String> = Map::default();

        // Aggiungi chiavi che si sovrappongono
        map.add("12", "twelve".to_string());
        map.add("123", "one-two-three".to_string());
        map.add("1234", "long".to_string());

        // Tutte dovrebbero essere accessibili
        assert_eq!(map.get("12"), Some(&"twelve".to_string()));
        assert_eq!(map.get("123"), Some(&"one-two-three".to_string()));
        assert_eq!(map.get("1234"), Some(&"long".to_string()));

        // I prefissi parziali non dovrebbero esistere se non esplicitamente aggiunti
        assert_eq!(map.get("1"), None);
    }

    #[test]
    fn test_map_overwrite_value() {
        let mut map: Map<String> = Map::default();

        map.add("42", "original".to_string());
        assert_eq!(map.get("42"), Some(&"original".to_string()));

        // Sovrascrivi con un nuovo valore
        map.add("42", "updated".to_string());
        assert_eq!(map.get("42"), Some(&"updated".to_string()));
    }

    #[test]
    fn test_map_empty_string() {
        let mut map: Map<String> = Map::default();

        // Aggiungi valore per stringa vuota (root)
        map.add("", "root_value".to_string());
        assert_eq!(map.get(""), Some(&"root_value".to_string()));
    }

    #[test]
    fn test_map_all_digits() {
        let mut map: Map<String> = Map::default();

        // Testa tutte le cifre da 0 a 9
        for i in 0..10 {
            let key = i.to_string();
            let value = format!("value_{}", i);
            map.add(&key, value.clone());
            assert_eq!(map.get(&key), Some(&value));
        }
    }

    #[test]
    fn test_map_long_key() {
        let mut map: Map<String> = Map::default();

        let long_key = "1234567890123456789";
        let value = "very_long_key".to_string();

        map.add(long_key, value.clone());
        assert_eq!(map.get(long_key), Some(&value));
    }

    #[test]
    fn test_map_different_types() {
        // Test con diversi tipi di valori
        let mut int_map: Map<i32> = Map::default();
        int_map.add("123", 42);
        assert_eq!(int_map.get("123"), Some(&42));

        let mut vec_map: Map<Vec<i32>> = Map::default();
        vec_map.add("456", vec![1, 2, 3]);
        assert_eq!(vec_map.get("456"), Some(&vec![1, 2, 3]));
    }

    #[test]
    #[should_panic(expected = "input must contain only digits")]
    fn test_map_invalid_input_add() {
        let mut map: Map<String> = Map::default();
        map.add("12a3", "invalid".to_string());
    }

    #[test]
    #[should_panic(expected = "input must contain only digits")]
    fn test_map_invalid_input_get() {
        let map: Map<String> = Map::default();
        map.get("a");
    }

    #[test]
    fn test_node_add_multiple_children() {
        let mut node: Node<String> = Node::default();

        // Aggiungi più figli
        node.add(1);
        node.add(5);
        node.add(9);

        // Verifica che esistano
        assert!(node.get(1).is_some());
        assert!(node.get(5).is_some());
        assert!(node.get(9).is_some());

        // Verifica che gli altri non esistano
        assert!(node.get(0).is_none());
        assert!(node.get(3).is_none());
        assert!(node.get(7).is_none());
    }

    #[test]
    fn test_node_add_same_digit_twice() {
        let mut node: Node<String> = Node::default();

        // Aggiungi lo stesso digit due volte
        node.add(3);
        node.add(3);

        // Verifica che esista solo un nodo per quel digit
        assert!(node.get(3).is_some());

        // Verifica che gli altri digit non esistano
        for i in 0..10 {
            if i != 3 {
                assert!(node.get(i).is_none());
            }
        }
    }

    #[test]
    fn test_complex_tree_structure() {
        let mut map: Map<String> = Map::default();

        // Costruisci una struttura complessa
        map.add("1", "one".to_string());
        map.add("10", "ten".to_string());
        map.add("100", "hundred".to_string());
        map.add("101", "one-oh-one".to_string());
        map.add("11", "eleven".to_string());
        map.add("2", "two".to_string());
        map.add("20", "twenty".to_string());

        // Verifica tutte le chiavi
        assert_eq!(map.get("1"), Some(&"one".to_string()));
        assert_eq!(map.get("10"), Some(&"ten".to_string()));
        assert_eq!(map.get("100"), Some(&"hundred".to_string()));
        assert_eq!(map.get("101"), Some(&"one-oh-one".to_string()));
        assert_eq!(map.get("11"), Some(&"eleven".to_string()));
        assert_eq!(map.get("2"), Some(&"two".to_string()));
        assert_eq!(map.get("20"), Some(&"twenty".to_string()));

        // Verifica che le chiavi parziali non esistano (se non esplicitamente aggiunte)
        assert_eq!(map.get("0"), None);
    }
}
