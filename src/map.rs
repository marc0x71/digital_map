#![allow(unused)]

use std::{iter, ops::Deref, thread::current};

use crate::error::MapError;

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
    fn add(&mut self, input: &str, value: T) -> Result<(), MapError> {
        let mut current = self.root.as_mut();
        for c in input.chars() {
            let digit = c.to_digit(10).ok_or(MapError::InvalidDigit(c))? as usize;
            current = current.add(digit);
        }
        current.set_value(value);
        Ok(())
    }

    fn get(&self, input: &str) -> Result<Option<&T>, MapError> {
        let mut current = self.root.as_ref();
        for c in input.chars() {
            let digit = c.to_digit(10).ok_or(MapError::InvalidDigit(c))? as usize;
            match current.get(digit) {
                Some(child) => current = child,
                None => return Ok(None),
            }
        }
        Ok(current.get_value())
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
    fn test_empty_map() {
        let map: Map<String> = Map::default();

        // Una mappa vuota dovrebbe restituire None per qualsiasi query
        assert_eq!(map.get("123").unwrap(), None);
        assert_eq!(map.get("0").unwrap(), None);
        assert_eq!(map.get("999").unwrap(), None);
    }

    #[test]
    fn test_single_insertion_and_retrieval() {
        let mut map: Map<String> = Map::default();

        // Inserimento di base
        map.add("123", "valore_123".to_string()).unwrap();

        // Verifica che il valore sia stato inserito correttamente
        assert_eq!(map.get("123").unwrap(), Some(&"valore_123".to_string()));

        // Verifica che altri valori restituiscano None
        assert_eq!(map.get("124").unwrap(), None);
        assert_eq!(map.get("12").unwrap(), None);
        assert_eq!(map.get("1234").unwrap(), None);
    }

    #[test]
    fn test_multiple_insertions() {
        let mut map: Map<i32> = Map::default();

        // Inserimenti multipli
        map.add("1", 10).unwrap();
        map.add("12", 120).unwrap();
        map.add("123", 1230).unwrap();
        map.add("124", 1240).unwrap();
        map.add("2", 20).unwrap();

        // Verifica tutti i valori
        assert_eq!(map.get("1").unwrap(), Some(&10));
        assert_eq!(map.get("12").unwrap(), Some(&120));
        assert_eq!(map.get("123").unwrap(), Some(&1230));
        assert_eq!(map.get("124").unwrap(), Some(&1240));
        assert_eq!(map.get("2").unwrap(), Some(&20));

        // Verifica che i percorsi non esistenti restituiscano None
        assert_eq!(map.get("13").unwrap(), None);
        assert_eq!(map.get("125").unwrap(), None);
        assert_eq!(map.get("3").unwrap(), None);
    }

    #[test]
    fn test_prefix_relationships() {
        let mut map: Map<&str> = Map::default();

        // Inserimento di stringhe con relazioni di prefisso
        map.add("12", "dodici").unwrap();
        map.add("1234", "milleduecentotrentaquattro").unwrap();
        map.add("12345", "dodicimilatrecentoquarantacinque")
            .unwrap();

        // Verifica che ogni percorso mantenga il suo valore specifico
        assert_eq!(map.get("12").unwrap(), Some(&"dodici"));
        assert_eq!(
            map.get("1234").unwrap(),
            Some(&"milleduecentotrentaquattro")
        );
        assert_eq!(
            map.get("12345").unwrap(),
            Some(&"dodicimilatrecentoquarantacinque")
        );

        // Verifica che i percorsi intermedi senza valori restituiscano None
        assert_eq!(map.get("1").unwrap(), None);
        assert_eq!(map.get("123").unwrap(), None);
    }

    #[test]
    fn test_overwrite_existing_value() {
        let mut map: Map<String> = Map::default();

        // Inserimento iniziale
        map.add("100", "cento".to_string()).unwrap();
        assert_eq!(map.get("100").unwrap(), Some(&"cento".to_string()));

        // Sovrascrittura dello stesso percorso
        map.add("100", "hundred".to_string()).unwrap();
        assert_eq!(map.get("100").unwrap(), Some(&"hundred".to_string()));
    }

    #[test]
    fn test_single_digit_keys() {
        let mut map: Map<char> = Map::default();

        // Test con tutte le cifre singole
        for i in 0..10 {
            map.add(&i.to_string(), char::from_digit(i as u32, 10).unwrap())
                .unwrap();
        }

        // Verifica tutti i valori
        for i in 0..10 {
            let expected_char = char::from_digit(i as u32, 10).unwrap();
            assert_eq!(map.get(&i.to_string()).unwrap(), Some(&expected_char));
        }
    }

    #[test]
    fn test_long_numeric_strings() {
        let mut map: Map<u64> = Map::default();

        // Test con stringhe numeriche molto lunghe
        let long_key = "12345678901234567890";
        let long_value = 12345678901234567890u64;

        map.add(long_key, long_value).unwrap();
        assert_eq!(map.get(long_key).unwrap(), Some(&long_value));

        // Verifica che prefissi della stringa lunga non abbiano valori
        assert_eq!(map.get("1234567890123456789").unwrap(), None);
        assert_eq!(map.get("123456789012345678901").unwrap(), None);
    }

    #[test]
    fn test_empty_string() {
        let mut map: Map<String> = Map::default();

        // Test con stringa vuota (dovrebbe funzionare e mappare alla radice)
        map.add("", "radice".to_string()).unwrap();
        assert_eq!(map.get("").unwrap(), Some(&"radice".to_string()));

        // Altri valori dovrebbero comunque funzionare normalmente
        map.add("1", "uno".to_string()).unwrap();
        assert_eq!(map.get("1").unwrap(), Some(&"uno".to_string()));
    }

    #[test]
    fn test_invalid_characters() {
        let mut map: Map<String> = Map::default();

        // Test con caratteri non numerici - dovrebbero generare errori
        assert!(map.add("12a3", "test".to_string()).is_err());
        assert!(map.add("hello", "test".to_string()).is_err());
        assert!(map.add("12.34", "test".to_string()).is_err());
        assert!(map.add("12-34", "test".to_string()).is_err());

        // Anche le query con caratteri invalidi dovrebbero fallire
        assert!(map.get("12a3").is_err());
        assert!(map.get("hello").is_err());
        assert!(map.get("12.34").is_err());
    }

    #[test]
    fn test_error_propagation() {
        let mut map: Map<String> = Map::default();
        map.add("123456", "test".to_string());

        // Test specifici per diversi tipi di caratteri invalidi
        match map.get("12a34") {
            Err(MapError::InvalidDigit('a')) => {} // Comportamento atteso
            _ => panic!("Dovrebbe restituire InvalidDigit('a')"),
        }

        match map.get("xyz") {
            Err(MapError::InvalidDigit('x')) => {} // Comportamento atteso
            _ => panic!("Dovrebbe restituire InvalidDigit('x')"),
        }
    }

    #[test]
    fn test_zero_padding() {
        let mut map: Map<String> = Map::default();

        // Test che stringhe con zeri iniziali siano trattate diversamente
        map.add("01", "zero-uno".to_string()).unwrap();
        map.add("1", "uno".to_string()).unwrap();
        map.add("001", "zero-zero-uno".to_string()).unwrap();

        // Ogni rappresentazione dovrebbe essere distinta
        assert_eq!(map.get("01").unwrap(), Some(&"zero-uno".to_string()));
        assert_eq!(map.get("1").unwrap(), Some(&"uno".to_string()));
        assert_eq!(map.get("001").unwrap(), Some(&"zero-zero-uno".to_string()));
    }

    #[test]
    fn test_node_schema_visualization() {
        let mut map: Map<i32> = Map::default();

        // Costruzione di una struttura specifica per testare schema()
        map.add("0", 0).unwrap();
        map.add("1", 1).unwrap();
        map.add("5", 5).unwrap();
        map.add("9", 9).unwrap();

        // Il schema della radice dovrebbe mostrare 'X' per le posizioni utilizzate
        let root_schema = map.root.schema();
        assert_eq!(root_schema.len(), 10);

        // Verifica posizioni specifiche
        assert_eq!(root_schema.chars().nth(0).unwrap(), 'X'); // posizione 0 usata
        assert_eq!(root_schema.chars().nth(1).unwrap(), 'X'); // posizione 1 usata
        assert_eq!(root_schema.chars().nth(2).unwrap(), '.'); // posizione 2 NON usata
        assert_eq!(root_schema.chars().nth(5).unwrap(), 'X'); // posizione 5 usata
        assert_eq!(root_schema.chars().nth(9).unwrap(), 'X'); // posizione 9 usata
    }

    #[test]
    fn test_complex_trie_structure() {
        let mut map: Map<String> = Map::default();

        // Costruzione di una struttura complessa con molti rami
        let test_data = vec![
            ("123", "percorso-123"),
            ("124", "percorso-124"),
            ("125", "percorso-125"),
            ("12", "percorso-12"),
            ("1", "percorso-1"),
            ("13", "percorso-13"),
            ("134", "percorso-134"),
            ("2", "percorso-2"),
            ("20", "percorso-20"),
            ("200", "percorso-200"),
        ];

        // Inserimento di tutti i dati di test
        for (key, value) in &test_data {
            map.add(key, value.to_string()).unwrap();
        }

        // Verifica di tutti i percorsi
        for (key, expected_value) in &test_data {
            assert_eq!(map.get(key).unwrap(), Some(&expected_value.to_string()));
        }

        // Verifica che percorsi non inseriti restituiscano None
        assert_eq!(map.get("126").unwrap(), None);
        assert_eq!(map.get("21").unwrap(), None);
        assert_eq!(map.get("201").unwrap(), None);
        assert_eq!(map.get("1234").unwrap(), None);
    }

    #[test]
    fn test_different_value_types() {
        // Test con diversi tipi di dati

        // Test con interi
        let mut int_map: Map<i32> = Map::default();
        int_map.add("42", 42).unwrap();
        assert_eq!(int_map.get("42").unwrap(), Some(&42));

        // Test con float
        let mut float_map: Map<f64> = Map::default();
        float_map.add("314", 3.14159).unwrap();
        assert_eq!(float_map.get("314").unwrap(), Some(&3.14159));

        // Test con vettori
        let mut vec_map: Map<Vec<i32>> = Map::default();
        vec_map.add("123", vec![1, 2, 3]).unwrap();
        assert_eq!(vec_map.get("123").unwrap(), Some(&vec![1, 2, 3]));

        // Test con strutture personalizzate
        #[derive(Debug, PartialEq, Default)]
        struct CustomStruct {
            name: String,
            value: i32,
        }

        let mut custom_map: Map<CustomStruct> = Map::default();
        let custom_value = CustomStruct {
            name: "test".to_string(),
            value: 100,
        };
        custom_map.add("999", custom_value).unwrap();

        let retrieved = custom_map.get("999").unwrap().unwrap();
        assert_eq!(retrieved.name, "test");
        assert_eq!(retrieved.value, 100);
    }
}
