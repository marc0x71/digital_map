#![allow(unused)]
use std::ops::Deref;
use std::str::FromStr;

use crate::error::TokenizerError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Single,
    AtLeastOne,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub digit: u8,
    pub kind: TokenKind,
}

impl Token {
    fn as_single(digit: u8) -> Self {
        Self {
            digit,
            kind: TokenKind::Single,
        }
    }

    fn as_maybe_one_or_more(digit: u8) -> Self {
        Self {
            digit,
            kind: TokenKind::AtLeastOne,
        }
    }

    fn change_kind(&mut self, kind: TokenKind) {
        self.kind = kind;
    }
}

#[derive(Debug, PartialEq)]
pub struct Tokens(Vec<Vec<Token>>);

impl Tokens {
    fn append_token(&mut self, t: Token) {
        for v in self.0.iter_mut() {
            v.push(t.clone());
        }
    }

    fn extend_tokens(&mut self, vt: Vec<Token>) {
        let mut new_vec = Vec::with_capacity(self.0.len() * vt.len());

        for current_tokens in self.0.iter() {
            for e in &vt {
                let mut new_tokens = current_tokens.clone();
                new_tokens.push(e.clone());
                new_vec.push(new_tokens);
            }
        }
        self.0 = new_vec;
    }

    fn extend_tokens_for_plus(&mut self, tokens: Vec<Token>) {
        let mut new_variants = Vec::with_capacity(self.0.len() * tokens.len() * 2);

        for current_sequence in &self.0 {
            for token in &tokens {
                let mut single_sequence = current_sequence.clone();
                single_sequence.push(Token::as_single(token.digit));
                new_variants.push(single_sequence);

                let mut maybe_sequence = current_sequence.clone();
                maybe_sequence.push(Token::as_maybe_one_or_more(token.digit));
                new_variants.push(maybe_sequence);
            }
        }

        self.0 = new_variants;
    }
}

impl Deref for Tokens {
    type Target = Vec<Vec<Token>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Tokens {
    type Item = Vec<Token>;
    type IntoIter = std::vec::IntoIter<Vec<Token>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tokens {
    type Item = &'a Vec<Token>;
    type IntoIter = std::slice::Iter<'a, Vec<Token>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Default for Tokens {
    fn default() -> Self {
        Self(vec![vec![]])
    }
}

impl FromStr for Tokens {
    type Err = TokenizerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Tokens::default();
        let mut chars = s.chars().peekable();
        println!("Tokens.from_str({s})");

        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    let digit = c.to_digit(10).ok_or(TokenizerError::InvalidDigit(c))? as u8;
                    result.append_token(Token::as_single(digit))
                }

                '[' => {
                    let mut v = vec![];

                    while let Some(digit_char) = chars.next_if(|c| *c != ']') {
                        let digit = digit_char
                            .to_digit(10)
                            .ok_or(TokenizerError::InvalidDigit(digit_char))?
                            as u8;
                        v.push(Token::as_single(digit));
                    }

                    chars.next().ok_or(TokenizerError::MissingClosingBracket)?;

                    if v.is_empty() {
                        return Err(TokenizerError::UnexpectedEmptyRange);
                    }

                    if let Some('*') = chars.peek() {
                        chars.next();

                        let mut zero_variants = Tokens(result.clone());

                        v.iter_mut()
                            .for_each(|e| e.change_kind(TokenKind::AtLeastOne));
                        zero_variants.extend_tokens(v);

                        result.0.extend(zero_variants);
                    } else if let Some('+') = chars.peek() {
                        chars.next();

                        result.extend_tokens_for_plus(v);
                    } else {
                        result.extend_tokens(v);
                    }
                }

                _ => return Err(TokenizerError::UnexpectedChar(c)),
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST BASE - DIGIT SINGOLI
    #[test]
    fn test_single_digits() {
        let tokens = "1234".parse::<Tokens>().unwrap();
        let expected = Tokens(vec![vec![
            Token {
                digit: 1,
                kind: TokenKind::Single,
            },
            Token {
                digit: 2,
                kind: TokenKind::Single,
            },
            Token {
                digit: 3,
                kind: TokenKind::Single,
            },
            Token {
                digit: 4,
                kind: TokenKind::Single,
            },
        ]]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_empty_string() {
        let tokens = "".parse::<Tokens>().unwrap();
        let expected = Tokens(vec![vec![]]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_single_digit() {
        let tokens = "5".parse::<Tokens>().unwrap();
        let expected = Tokens(vec![vec![Token {
            digit: 5,
            kind: TokenKind::Single,
        }]]);
        assert_eq!(tokens, expected);
    }

    // TEST PARENTESI SEMPLICI
    #[test]
    fn test_simple_brackets() {
        let tokens = "[34]".parse::<Tokens>().unwrap();
        let expected = Tokens(vec![
            vec![Token {
                digit: 3,
                kind: TokenKind::Single,
            }],
            vec![Token {
                digit: 4,
                kind: TokenKind::Single,
            }],
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_brackets_with_context() {
        let tokens = "12[34]56".parse::<Tokens>().unwrap();
        let expected = Tokens(vec![
            vec![
                Token {
                    digit: 1,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 2,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 3,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 5,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 6,
                    kind: TokenKind::Single,
                },
            ],
            vec![
                Token {
                    digit: 1,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 2,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 4,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 5,
                    kind: TokenKind::Single,
                },
                Token {
                    digit: 6,
                    kind: TokenKind::Single,
                },
            ],
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_multiple_brackets() {
        let tokens = "[12][34]".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 4);

        let digits: Vec<Vec<u8>> = tokens
            .iter()
            .map(|variant| variant.iter().map(|t| t.digit).collect())
            .collect();

        assert!(digits.contains(&vec![1, 3]));
        assert!(digits.contains(&vec![1, 4]));
        assert!(digits.contains(&vec![2, 3]));
        assert!(digits.contains(&vec![2, 4]));
    }

    // TEST ASTERISCO (*)
    #[test]
    fn test_asterisk_single_digit() {
        let tokens = "12[3]*4".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 2);

        // Prima sequenza: salta il 3 (zero occorrenze)
        let seq1 = &tokens[0];
        assert_eq!(seq1.len(), 3);
        assert_eq!(
            seq1[0],
            Token {
                digit: 1,
                kind: TokenKind::Single
            }
        );
        assert_eq!(
            seq1[1],
            Token {
                digit: 2,
                kind: TokenKind::Single
            }
        );
        assert_eq!(
            seq1[2],
            Token {
                digit: 4,
                kind: TokenKind::Single
            }
        );

        // Seconda sequenza: include 3 come AtLeastOne
        let seq2 = &tokens[1];
        assert_eq!(seq2.len(), 4);
        assert_eq!(
            seq2[0],
            Token {
                digit: 1,
                kind: TokenKind::Single
            }
        );
        assert_eq!(
            seq2[1],
            Token {
                digit: 2,
                kind: TokenKind::Single
            }
        );
        assert_eq!(
            seq2[2],
            Token {
                digit: 3,
                kind: TokenKind::AtLeastOne
            }
        );
        assert_eq!(
            seq2[3],
            Token {
                digit: 4,
                kind: TokenKind::Single
            }
        );
    }

    #[test]
    fn test_asterisk_multiple_digits() {
        let tokens = "1[23]*4".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 3); // 1 + 2 (una per zero occorrenze, due per i digit ripetibili)

        // Prima sequenza: salta completamente [23]
        let zero_seq = &tokens[0];
        assert_eq!(
            zero_seq,
            &vec![
                Token {
                    digit: 1,
                    kind: TokenKind::Single
                },
                Token {
                    digit: 4,
                    kind: TokenKind::Single
                },
            ]
        );

        // Seconda e terza sequenza: con 2 e 3 ripetibili
        let remaining_seqs: Vec<Vec<u8>> = tokens[1..]
            .iter()
            .map(|seq| seq.iter().map(|t| t.digit).collect())
            .collect();

        assert!(remaining_seqs.contains(&vec![1, 2, 4]));
        assert!(remaining_seqs.contains(&vec![1, 3, 4]));

        // Verifica che i digit centrali siano AtLeastOne
        assert_eq!(tokens[1][1].kind, TokenKind::AtLeastOne);
        assert_eq!(tokens[2][1].kind, TokenKind::AtLeastOne);
    }

    #[test]
    fn test_asterisk_at_end() {
        let tokens = "12[3]*".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 2);

        // Sequenza che termina senza il 3
        assert_eq!(
            tokens[0],
            vec![
                Token {
                    digit: 1,
                    kind: TokenKind::Single
                },
                Token {
                    digit: 2,
                    kind: TokenKind::Single
                },
            ]
        );

        // Sequenza che include 3 ripetibile
        assert_eq!(
            tokens[1],
            vec![
                Token {
                    digit: 1,
                    kind: TokenKind::Single
                },
                Token {
                    digit: 2,
                    kind: TokenKind::Single
                },
                Token {
                    digit: 3,
                    kind: TokenKind::AtLeastOne
                },
            ]
        );
    }

    #[test]
    fn test_asterisk_only() {
        let tokens = "[5]*".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 2);

        // Sequenza vuota (zero occorrenze)
        assert_eq!(tokens[0], vec![]);

        // Sequenza con 5 ripetibile
        assert_eq!(
            tokens[1],
            vec![Token {
                digit: 5,
                kind: TokenKind::AtLeastOne
            },]
        );
    }

    // TEST PLUS (+)
    #[test]
    fn test_plus_single_digit() {
        let tokens = "12[3]+4".parse::<Tokens>().unwrap();
        dbg!(&tokens);

        assert_eq!(tokens.len(), 2);

        // Prima sequenza: esattamente uno
        let seq1 = &tokens[0];
        assert_eq!(
            seq1[2],
            Token {
                digit: 3,
                kind: TokenKind::Single
            }
        );

        // Seconda sequenza: zero o più
        let seq2 = &tokens[1];
        assert_eq!(
            seq2[2],
            Token {
                digit: 3,
                kind: TokenKind::AtLeastOne
            }
        );

        // Entrambe dovrebbero avere la stessa struttura base
        for seq in &tokens {
            assert_eq!(seq.len(), 4);
            assert_eq!(
                seq[0],
                Token {
                    digit: 1,
                    kind: TokenKind::Single
                }
            );
            assert_eq!(
                seq[1],
                Token {
                    digit: 2,
                    kind: TokenKind::Single
                }
            );
            assert_eq!(
                seq[3],
                Token {
                    digit: 4,
                    kind: TokenKind::Single
                }
            );
        }
    }

    #[test]
    fn test_plus_multiple_digits() {
        let tokens = "1[23]+4".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 4); // 2 digits × 2 tipi = 4 sequenze

        // Conta i tipi di token per il digit centrale
        let central_kinds: Vec<TokenKind> = tokens.iter().map(|seq| seq[1].kind.clone()).collect();

        let single_count = central_kinds
            .iter()
            .filter(|&kind| *kind == TokenKind::Single)
            .count();
        let at_least_one_count = central_kinds
            .iter()
            .filter(|&kind| *kind == TokenKind::AtLeastOne)
            .count();

        assert_eq!(single_count, 2);
        assert_eq!(at_least_one_count, 2);

        // Verifica che abbiamo tutte le combinazioni di digit
        let central_digits: Vec<u8> = tokens.iter().map(|seq| seq[1].digit).collect();

        assert_eq!(central_digits.iter().filter(|&&d| d == 2).count(), 2);
        assert_eq!(central_digits.iter().filter(|&&d| d == 3).count(), 2);
    }

    #[test]
    fn test_plus_at_end() {
        let tokens = "12[3]+".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 2);

        // Prima sequenza: Single
        assert_eq!(tokens[0][2].kind, TokenKind::Single);

        // Seconda sequenza: AtLeastOne
        assert_eq!(tokens[1][2].kind, TokenKind::AtLeastOne);
    }

    #[test]
    fn test_plus_only() {
        let tokens = "[7]+".parse::<Tokens>().unwrap();

        assert_eq!(tokens.len(), 2);

        assert_eq!(
            tokens[0],
            vec![Token {
                digit: 7,
                kind: TokenKind::Single
            },]
        );

        assert_eq!(
            tokens[1],
            vec![Token {
                digit: 7,
                kind: TokenKind::AtLeastOne
            },]
        );
    }

    // TEST PATTERN COMPLESSI
    #[test]
    fn test_mixed_patterns() {
        let tokens = "1[23]*[45]+6".parse::<Tokens>().unwrap();

        // Dovrebbe generare multiple combinazioni
        // [23]* genera 3 varianti (0, 2, 3)
        // [45]+ genera 4 varianti (2×2)
        // Totale: 3 × 4 = 12 varianti
        assert_eq!(tokens.len(), 12);

        // Verifica che tutti inizino con 1 e finiscano con 6
        for seq in &tokens {
            assert_eq!(seq.first().unwrap().digit, 1);
            assert_eq!(seq.last().unwrap().digit, 6);
        }
    }

    #[test]
    fn test_consecutive_brackets() {
        let tokens = "[12][34][56]".parse::<Tokens>().unwrap();

        // 2 × 2 × 2 = 8 combinazioni
        assert_eq!(tokens.len(), 8);

        // Verifica alcune combinazioni specifiche
        let digits: Vec<Vec<u8>> = tokens
            .iter()
            .map(|seq| seq.iter().map(|t| t.digit).collect())
            .collect();

        assert!(digits.contains(&vec![1, 3, 5]));
        assert!(digits.contains(&vec![2, 4, 6]));
        assert!(digits.contains(&vec![1, 4, 6]));
    }

    // TEST ERRORI
    #[test]
    fn test_invalid_digit_outside_brackets() {
        let result = "123a4".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::UnexpectedChar('a'))));
    }

    #[test]
    fn test_invalid_digit_inside_brackets() {
        let result = "123[45x67]".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::InvalidDigit('x'))));
    }

    #[test]
    fn test_missing_closing_bracket() {
        let result = "123[456".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::MissingClosingBracket)));
    }

    #[test]
    fn test_empty_brackets() {
        let result = "123[]456".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::UnexpectedEmptyRange)));
    }

    #[test]
    fn test_unexpected_characters() {
        let result = "123&456".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::UnexpectedChar('&'))));

        let result = "123@456".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::UnexpectedChar('@'))));
    }

    #[test]
    fn test_modifiers_without_brackets() {
        let result = "123*456".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::UnexpectedChar('*'))));

        let result = "123+456".parse::<Tokens>();
        assert!(matches!(result, Err(TokenizerError::UnexpectedChar('+'))));
    }

    // TEST EDGE CASES
    #[test]
    fn test_all_digits() {
        let tokens = "[0123456789]".parse::<Tokens>().unwrap();
        assert_eq!(tokens.len(), 10);

        let digits: Vec<u8> = tokens.iter().map(|seq| seq[0].digit).collect();

        for i in 0..10 {
            assert!(digits.contains(&(i as u8)));
        }
    }

    #[test]
    fn test_repeated_digits_in_brackets() {
        let tokens = "[333]".parse::<Tokens>().unwrap();
        assert_eq!(tokens.len(), 3);

        // Tutti dovrebbero essere digit 3
        for seq in &tokens {
            assert_eq!(seq[0].digit, 3);
        }
    }

    #[test]
    fn test_single_digit_brackets() {
        let tokens = "[5]".parse::<Tokens>().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0],
            vec![Token {
                digit: 5,
                kind: TokenKind::Single
            },]
        );
    }

    // TEST HELPER METHODS
    #[test]
    fn test_token_constructors() {
        let single = Token::as_single(7);
        assert_eq!(single.digit, 7);
        assert_eq!(single.kind, TokenKind::Single);

        let maybe = Token::as_maybe_one_or_more(3);
        assert_eq!(maybe.digit, 3);
        assert_eq!(maybe.kind, TokenKind::AtLeastOne);
    }

    #[test]
    fn test_token_change_kind() {
        let mut token = Token::as_single(5);
        assert_eq!(token.kind, TokenKind::Single);

        token.change_kind(TokenKind::AtLeastOne);
        assert_eq!(token.kind, TokenKind::AtLeastOne);
    }

    // TEST ITERATORI E DEREF
    #[test]
    fn test_deref_functionality() {
        let tokens = "12[34]".parse::<Tokens>().unwrap();

        // Test Deref
        assert_eq!(tokens.len(), 2);
        assert!(!tokens.is_empty());

        // Test indexing
        assert_eq!(tokens[0].len(), 3);
        assert_eq!(tokens[1].len(), 3);
    }

    #[test]
    fn test_into_iterator() {
        let tokens = "1[23]".parse::<Tokens>().unwrap();

        let mut count = 0;
        for variant in tokens {
            count += 1;
            assert_eq!(variant.len(), 2);
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_iterator_ref() {
        let tokens = "1[23]".parse::<Tokens>().unwrap();

        let mut count = 0;
        for variant in &tokens {
            count += 1;
            assert_eq!(variant.len(), 2);
        }
        assert_eq!(count, 2);

        // tokens è ancora utilizzabile dopo l'iterazione
        assert_eq!(tokens.len(), 2);
    }

    // TEST REGRESSIONE
    #[test]
    fn test_asterisk_vs_plus_difference() {
        let asterisk_tokens = "12[3]*4".parse::<Tokens>().unwrap();
        let plus_tokens = "12[3]+4".parse::<Tokens>().unwrap();

        // Entrambi dovrebbero avere 2 sequenze
        assert_eq!(asterisk_tokens.len(), 2);
        assert_eq!(plus_tokens.len(), 2);

        // Ma con logiche diverse
        // Asterisk: sequenza senza 3, sequenza con 3 ripetibile
        assert_eq!(asterisk_tokens[0].len(), 3); // 1,2,4
        assert_eq!(asterisk_tokens[1].len(), 4); // 1,2,3*,4

        // Plus: entrambe le sequenze hanno 4 elementi
        assert_eq!(plus_tokens[0].len(), 4); // 1,2,3,4
        assert_eq!(plus_tokens[1].len(), 4); // 1,2,3*,4

        // Plus ha sempre il digit, asterisk può non averlo
        assert!(!asterisk_tokens[0].iter().any(|t| t.digit == 3));
        assert!(plus_tokens[0].iter().any(|t| t.digit == 3));
    }
}
