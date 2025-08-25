#![allow(unused)]

use std::fmt::Debug;
use std::{iter, ops::Deref, thread::current};

use crate::{
    error::{MapError, TokenizerError},
    node::Node,
    tokenizer::{Token, TokenKind, Tokens},
};

#[derive(Debug)]
struct Map<T> {
    root: Box<Node<T>>,
}

impl<T> Map<T>
where
    T: Default + Clone + Debug + PartialEq,
{
    fn add(&mut self, input: &str, value: T) -> Result<(), MapError> {
        let tokens_list = input.parse::<Tokens>()?;
        // println!("{:?}", tokens_list);
        println!("pattern {input}");
        dbg!(&tokens_list);
        let mut current = self.root.as_mut();
        for tokens in tokens_list.iter() {
            current = self.root.as_mut();
            for token in tokens {
                current = current.add_with(token.digit, token.into());
            }
            if let Some(current_value) = current.get_value()
                && *current_value != value
            {
                return Err(MapError::PathAlreadyExists);
            }
            current.set_value(value.clone());
        }
        Ok(())
    }

    fn get(&self, input: &str) -> Result<Option<&T>, MapError> {
        println!("Map.get({input})");
        let mut current = self.root.as_ref();
        for c in input.chars() {
            let digit = c.to_digit(10).ok_or(MapError::InvalidDigit(c))? as u8;
            match current.get(digit) {
                Some(node) => current = node,

                None => {
                    return Ok(None);
                }
            }
        }
        print!(">>>>>>>>>>>");
        current.dump(1);
        Ok(current.get_value())
    }

    fn dump(&self) {
        self.root.dump(0);
    }
}

impl<T> Default for Map<T>
where
    T: Default + Debug,
{
    fn default() -> Self {
        Self {
            root: Box::new(Node::root()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST BASE - FUNZIONALITÀ SEMPLICE
    #[test]
    fn test_map_default() {
        let map: Map<String> = Map::default();
        // Verifica che la mappa sia inizializzata con nodo root
        // Non possiamo accedere direttamente ai campi privati,
        // ma possiamo testare il comportamento
        assert_eq!(map.get("").unwrap(), None);
    }

    #[test]
    fn test_add_and_get_simple() {
        let mut map: Map<String> = Map::default();

        map.add("123", "test_value".to_string()).unwrap();
        let result = map.get("123").unwrap();

        assert_eq!(result, Some(&"test_value".to_string()));
    }

    #[test]
    fn test_get_non_existing() {
        let mut map: Map<String> = Map::default();

        map.add("123", "exists".to_string()).unwrap();

        assert_eq!(map.get("124").unwrap(), None);
        assert_eq!(map.get("12").unwrap(), None);
        assert_eq!(map.get("1234").unwrap(), None);
        assert_eq!(map.get("456").unwrap(), None);
    }

    #[test]
    fn test_empty_string_pattern() {
        let mut map: Map<String> = Map::default();

        map.add("", "empty_pattern".to_string()).unwrap();
        assert_eq!(map.get("").unwrap(), Some(&"empty_pattern".to_string()));
    }

    // TEST PATTERN CON PARENTESI SEMPLICI
    #[test]
    fn test_bracket_variants() {
        let mut map: Map<String> = Map::default();

        map.add("12[34]5", "variant".to_string()).unwrap();

        // Dovrebbe trovare entrambe le varianti
        assert_eq!(map.get("1235").unwrap(), Some(&"variant".to_string()));
        assert_eq!(map.get("1245").unwrap(), Some(&"variant".to_string()));

        // Ma non altre combinazioni
        assert_eq!(map.get("1225").unwrap(), None);
        assert_eq!(map.get("1255").unwrap(), None);
        assert_eq!(map.get("125").unwrap(), None);
    }

    #[test]
    fn test_multiple_brackets() {
        let mut map: Map<String> = Map::default();

        map.add("[12][34]", "multi".to_string()).unwrap();

        // Verifica tutte le combinazioni
        assert_eq!(map.get("13").unwrap(), Some(&"multi".to_string()));
        assert_eq!(map.get("14").unwrap(), Some(&"multi".to_string()));
        assert_eq!(map.get("23").unwrap(), Some(&"multi".to_string()));
        assert_eq!(map.get("24").unwrap(), Some(&"multi".to_string()));

        // Verifica che non ci siano altre combinazioni
        assert_eq!(map.get("11").unwrap(), None);
        assert_eq!(map.get("15").unwrap(), None);
        assert_eq!(map.get("12").unwrap(), None);
    }

    // TEST ASTERISCO (zero o più)
    #[test]
    fn test_asterisk_basic() {
        let mut map: Map<String> = Map::default();

        map.add("12[3]*4", "repeating".to_string()).unwrap();

        // Zero occorrenze (salta il 3)
        assert_eq!(map.get("124").unwrap(), Some(&"repeating".to_string()));

        // Una occorrenza
        assert_eq!(map.get("1234").unwrap(), Some(&"repeating".to_string()));

        // Multiple occorrenze
        assert_eq!(map.get("12334").unwrap(), Some(&"repeating".to_string()));
        assert_eq!(map.get("123334").unwrap(), Some(&"repeating".to_string()));
    }

    #[test]
    fn test_asterisk_multiple_digits() {
        let mut map: Map<String> = Map::default();

        map.add("1[23]*4", "multi_repeat".to_string()).unwrap();

        // Zero occorrenze
        assert_eq!(map.get("14").unwrap(), Some(&"multi_repeat".to_string()));

        // Singole occorrenze
        assert_eq!(map.get("124").unwrap(), Some(&"multi_repeat".to_string()));
        assert_eq!(map.get("134").unwrap(), Some(&"multi_repeat".to_string()));

        // Multiple occorrenze dello stesso digit
        assert_eq!(map.get("1224").unwrap(), Some(&"multi_repeat".to_string()));
        assert_eq!(map.get("1334").unwrap(), Some(&"multi_repeat".to_string()));

        // Mix non supportato (i pattern sono indipendenti)
        assert_eq!(map.get("1234").unwrap(), None);
        assert_eq!(map.get("1324").unwrap(), None);
    }

    #[test]
    fn test_asterisk_at_end() {
        let mut map: Map<String> = Map::default();

        map.add("12[3]*", "end_repeat".to_string()).unwrap();

        // Zero occorrenze
        assert_eq!(map.get("12").unwrap(), Some(&"end_repeat".to_string()));

        // Multiple occorrenze
        assert_eq!(map.get("123").unwrap(), Some(&"end_repeat".to_string()));
        assert_eq!(map.get("1233").unwrap(), Some(&"end_repeat".to_string()));
        assert_eq!(map.get("12333").unwrap(), Some(&"end_repeat".to_string()));
    }

    // TEST PLUS (almeno uno)
    #[test]
    fn test_plus_basic() {
        let mut map: Map<String> = Map::default();

        map.add("12[3]+4", "at_least_one".to_string()).unwrap();
        map.dump();

        // Zero occorrenze - NON dovrebbe trovare
        assert_eq!(map.get("124").unwrap(), None);

        // Una occorrenza - dovrebbe trovare
        assert_eq!(map.get("1234").unwrap(), Some(&"at_least_one".to_string()));

        // Multiple occorrenze - dovrebbe trovare
        assert_eq!(map.get("12334").unwrap(), Some(&"at_least_one".to_string()));
        assert_eq!(
            map.get("123334").unwrap(),
            Some(&"at_least_one".to_string())
        );
    }

    #[test]
    fn test_plus_multiple_digits() {
        let mut map: Map<String> = Map::default();

        map.add("1[23]+4", "multi_at_least_one".to_string())
            .unwrap();
        map.dump();

        // Zero occorrenze - NON dovrebbe trovare
        assert_eq!(map.get("14").unwrap(), None);

        // Singole occorrenze - dovrebbe trovare
        assert_eq!(
            map.get("124").unwrap(),
            Some(&"multi_at_least_one".to_string())
        );
        assert_eq!(
            map.get("134").unwrap(),
            Some(&"multi_at_least_one".to_string())
        );

        // Multiple occorrenze dello stesso digit - dovrebbe trovare
        assert_eq!(
            map.get("1224").unwrap(),
            Some(&"multi_at_least_one".to_string())
        );
        assert_eq!(
            map.get("1334").unwrap(),
            Some(&"multi_at_least_one".to_string())
        );

        // Mix di digit diversi - NON supportato
        assert_eq!(map.get("1234").unwrap(), None);
        assert_eq!(map.get("1324").unwrap(), None);
    }

    #[test]
    fn test_plus_at_end() {
        let mut map: Map<String> = Map::default();

        map.add("12[3]+", "end_at_least_one".to_string()).unwrap();

        // Zero occorrenze - NON dovrebbe trovare
        assert_eq!(map.get("12").unwrap(), None);

        // Una o più occorrenze - dovrebbe trovare
        assert_eq!(
            map.get("123").unwrap(),
            Some(&"end_at_least_one".to_string())
        );
        assert_eq!(
            map.get("1233").unwrap(),
            Some(&"end_at_least_one".to_string())
        );
        assert_eq!(
            map.get("12333").unwrap(),
            Some(&"end_at_least_one".to_string())
        );
    }

    // TEST PATTERN COMPLESSI
    #[test]
    fn test_complex_mixed_pattern() {
        let mut map: Map<String> = Map::default();

        map.add("1[23]*[45]+6", "complex".to_string()).unwrap();

        // Con ripetizioni di [45]+
        assert_eq!(map.get("14446").unwrap(), Some(&"complex".to_string()));
        assert_eq!(map.get("15556").unwrap(), Some(&"complex".to_string()));
        assert_eq!(map.get("124446").unwrap(), Some(&"complex".to_string()));
        assert_eq!(map.get("134446").unwrap(), Some(&"complex".to_string()));
        assert_eq!(map.get("125556").unwrap(), Some(&"complex".to_string()));
        assert_eq!(map.get("134446").unwrap(), Some(&"complex".to_string()));

        // Casi non validi - manca [45]+
        assert_eq!(map.get("16").unwrap(), None);
        assert_eq!(map.get("126").unwrap(), None);
    }

    #[test]
    fn test_consecutive_modifiers() {
        let mut map: Map<String> = Map::default();

        map.add("[1]*[2]+[3]*", "consecutive".to_string()).unwrap();

        // Solo il [2]+ è obbligatorio
        assert_eq!(map.get("2").unwrap(), Some(&"consecutive".to_string()));
        assert_eq!(map.get("22").unwrap(), Some(&"consecutive".to_string()));

        // Con [1]* opzionale
        assert_eq!(map.get("12").unwrap(), Some(&"consecutive".to_string()));
        assert_eq!(map.get("1112").unwrap(), Some(&"consecutive".to_string()));

        // Con [3]* opzionale
        assert_eq!(map.get("23").unwrap(), Some(&"consecutive".to_string()));
        assert_eq!(map.get("2333").unwrap(), Some(&"consecutive".to_string()));

        // Combinazioni complete
        assert_eq!(
            map.get("11223333").unwrap(),
            Some(&"consecutive".to_string())
        );
    }

    // TEST ERRORI
    #[test]
    fn test_add_invalid_pattern() {
        let mut map: Map<String> = Map::default();

        let result = map.add("12[3a4]5", "invalid".to_string());
        assert!(result.is_err());

        match result.unwrap_err() {
            MapError::TokenizeError(TokenizerError::InvalidDigit('a')) => {}
            other => panic!("Expected InvalidDigit('a'), got {:?}", other),
        }
    }

    #[test]
    fn test_add_empty_brackets() {
        let mut map: Map<String> = Map::default();

        let result = map.add("12[]5", "empty".to_string());
        assert!(result.is_err());

        match result.unwrap_err() {
            MapError::TokenizeError(TokenizerError::UnexpectedEmptyRange) => {}
            other => panic!("Expected UnexpectedEmptyRange, got {:?}", other),
        }
    }

    #[test]
    fn test_add_missing_bracket() {
        let mut map: Map<String> = Map::default();

        let result = map.add("12[345", "missing".to_string());
        assert!(result.is_err());

        match result.unwrap_err() {
            MapError::TokenizeError(TokenizerError::MissingClosingBracket) => {}
            other => panic!("Expected MissingClosingBracket, got {:?}", other),
        }
    }

    #[test]
    fn test_get_invalid_input() {
        let mut map: Map<String> = Map::default();

        // Aggiungi un pattern che copre l'input fino al carattere problematico
        map.add("12", "partial".to_string()).unwrap();

        let result = map.get("12a3");
        assert!(result.is_err());

        match result.unwrap_err() {
            MapError::InvalidDigit('a') => {}
            other => panic!("Expected InvalidDigit('a'), got {:?}", other),
        }
    }

    #[test]
    fn test_get_invalid_input_at_start() {
        let map: Map<String> = Map::default();

        let result = map.get("x123");
        assert!(result.is_err());

        match result.unwrap_err() {
            MapError::InvalidDigit('x') => {}
            other => panic!("Expected InvalidDigit('x'), got {:?}", other),
        }
    }

    // TEST EDGE CASES
    #[test]
    fn test_all_digits() {
        let mut map: Map<String> = Map::default();

        map.add("[0123456789]", "any_digit".to_string()).unwrap();

        for i in 0..10 {
            let input = i.to_string();
            assert_eq!(map.get(&input).unwrap(), Some(&"any_digit".to_string()));
        }
    }

    #[test]
    fn test_long_repetition() {
        let mut map: Map<String> = Map::default();

        map.add("1[2]*3", "long_repeat".to_string()).unwrap();

        // Testa con una lunga sequenza di ripetizioni
        let long_input = format!("1{}3", "2".repeat(100));
        assert_eq!(
            map.get(&long_input).unwrap(),
            Some(&"long_repeat".to_string())
        );
    }

    #[test]
    fn test_multiple_patterns_same_start() {
        let mut map: Map<String> = Map::default();

        map.add("123", "exact".to_string()).unwrap();
        map.add("12[4]", "variant".to_string()).unwrap();
        map.add("12[5]*", "optional".to_string()).unwrap();

        // Ogni pattern dovrebbe essere accessibile
        assert_eq!(map.get("123").unwrap(), Some(&"exact".to_string()));
        assert_eq!(map.get("124").unwrap(), Some(&"variant".to_string()));
        assert_eq!(map.get("12").unwrap(), Some(&"optional".to_string())); // [5]* con 0 occorrenze
        assert_eq!(map.get("125").unwrap(), Some(&"optional".to_string())); // [5]* con 1 occorrenza
    }

    // TEST DIVERSI TIPI DI VALORE
    #[test]
    fn test_different_value_types() {
        let mut int_map: Map<i32> = Map::default();
        int_map.add("123", 42).unwrap();
        assert_eq!(int_map.get("123").unwrap(), Some(&42));

        let mut vec_map: Map<Vec<i32>> = Map::default();
        vec_map.add("456", vec![1, 2, 3]).unwrap();
        assert_eq!(vec_map.get("456").unwrap(), Some(&vec![1, 2, 3]));

        let mut bool_map: Map<bool> = Map::default();
        bool_map.add("789", true).unwrap();
        assert_eq!(bool_map.get("789").unwrap(), Some(&true));
    }

    // TEST CASI SPECIFICI PER LA NUOVA ARCHITETTURA
    #[test]
    fn test_asterisk_creates_correct_paths() {
        let mut map: Map<String> = Map::default();

        map.add("[3]*", "root_repeat".to_string()).unwrap();

        // Dovrebbe creare due percorsi dal root:
        // 1. Percorso vuoto (zero occorrenze)
        // 2. Percorso con 3 ripetibile (una o più occorrenze)

        assert_eq!(map.get("").unwrap(), Some(&"root_repeat".to_string()));
        assert_eq!(map.get("3").unwrap(), Some(&"root_repeat".to_string()));
        assert_eq!(map.get("333").unwrap(), Some(&"root_repeat".to_string()));
    }

    #[test]
    fn test_plus_creates_correct_paths() {
        let mut map: Map<String> = Map::default();

        map.add("[4]+", "root_at_least_one".to_string()).unwrap();

        // Dovrebbe creare due percorsi:
        // 1. 4 esattamente una volta
        // 2. 4 ripetibile (una o più occorrenze)

        assert_eq!(map.get("").unwrap(), None); // Zero occorrenze non permesse
        assert_eq!(
            map.get("4").unwrap(),
            Some(&"root_at_least_one".to_string())
        );
        assert_eq!(
            map.get("444").unwrap(),
            Some(&"root_at_least_one".to_string())
        );
    }

    #[test]
    fn test_pattern_conflict_detection() {
        let mut map: Map<String> = Map::default();

        map.add("1[23]*4", "pattern_a".to_string()).unwrap();

        // Questo dovrebbe fallire perché genera percorsi sovrapposti
        let result = map.add("1[23]+4", "pattern_b".to_string());
        assert!(matches!(result, Err(MapError::PathAlreadyExists)));
    }

    #[test]
    fn test_pattern_independence_corrected() {
        let mut map: Map<String> = Map::default();

        // Pattern che NON si sovrappongono
        map.add("1[2]*3", "pattern_a".to_string()).unwrap();
        map.add("1[3]+4", "pattern_b".to_string()).unwrap();

        assert_eq!(map.get("13").unwrap(), Some(&"pattern_a".to_string()));
        assert_eq!(map.get("134").unwrap(), Some(&"pattern_b".to_string()));
    }
    #[test]
    fn test_debug_output() {
        let mut map: Map<String> = Map::default();
        map.add("123", "debug_test".to_string()).unwrap();

        // Test che dump() non causi panic (non possiamo testare l'output facilmente)
        map.dump(); // Dovrebbe stampare la struttura interna

        // Test get() con dump interno (dovrebbe funzionare senza errori)
        let result = map.get("123").unwrap();
        assert_eq!(result, Some(&"debug_test".to_string()));
    }
}
