# Pattern Trie

Una struttura dati efficiente in Rust per mappare pattern numerici a valori, implementata come trie digitale con supporto per espressioni regolari semplici.

## Descrizione

`Pattern Trie` è un trie (albero di prefissi) specializzato per pattern numerici che supporta sintassi simile alle regex per cifre numeriche (0-9). Questa struttura dati permette di definire pattern flessibili e recuperare valori associati in modo efficiente, con complessità temporale O(k) dove k è la lunghezza dell'input.

## Caratteristiche

- **Pattern flessibili**: Supporta varianti `[34]`, ripetizioni `[3]*` e obbligatorietà `[3]+`
- **Efficiente**: Inserimento e ricerca in tempo O(k)
- **Type-safe**: Supporta qualsiasi tipo di valore che implementi `Default + Clone + Debug`
- **Gestione errori robusta**: Validazione completa con messaggi informativi
- **Memory-efficient**: Condivide percorsi comuni e gestisce pattern sovrapposti

## Sintassi Supportata

- **Digit singoli**: `123` → corrisponde esattamente a "123"
- **Varianti**: `12[34]5` → corrisponde a "1235" o "1245"  
- **Zero o più**: `12[3]*4` → corrisponde a "124", "1234", "12334", etc.
- **Almeno uno**: `12[3]+4` → corrisponde a "1234", "12334", etc. (ma non "124")
- **Combinazioni**: `1[23]*[45]+6` → pattern complessi con multiple regole

## Utilizzo

```rust
use pattern_trie::PatternTrie;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut trie: PatternTrie<String> = PatternTrie::default();
    
    // Pattern semplici
    trie.add("123", "numero fisso".to_string())?;
    
    // Pattern con varianti
    trie.add("12[34]5", "varianti centrali".to_string())?;
    
    // Pattern con ripetizioni
    trie.add("1[2]*3", "zeri o più 2".to_string())?;
    trie.add("1[4]+5", "almeno un 4".to_string())?;
    
    // Ricerca
    println!("{:?}", trie.get("123")?);     // Some("numero fisso")
    println!("{:?}", trie.get("1235")?);    // Some("varianti centrali")
    println!("{:?}", trie.get("1245")?);    // Some("varianti centrali")
    println!("{:?}", trie.get("13")?);      // Some("zeri o più 2")
    println!("{:?}", trie.get("1223")?);    // Some("zeri o più 2")
    println!("{:?}", trie.get("145")?);     // Some("almeno un 4")
    println!("{:?}", trie.get("15")?);      // None (manca il 4 obbligatorio)
    
    Ok(())
}
```

## Esempi Avanzati

### Routing Telefonico

```rust
let mut routing: PatternTrie<String> = PatternTrie::default();

// Prefissi nazionali
routing.add("39[0-9]*", "Italia".to_string())?;
routing.add("1[2-9]*", "Nord America".to_string())?;
routing.add("44[0-9]*", "Regno Unito".to_string())?;

// Numeri di emergenza
routing.add("11[28]", "Emergenze".to_string())?;

assert_eq!(routing.get("393451234567")?, Some(&"Italia".to_string()));
assert_eq!(routing.get("112")?, Some(&"Emergenze".to_string()));
```

### Validazione Codici

```rust
let mut validator: PatternTrie<bool> = PatternTrie::default();

// Codici postali italiani
validator.add("[0-9][0-9][0-9][0-9][0-9]", true)?;

// Codici prodotto con pattern specifico
validator.add("ABC[1-3]+[0-9]*", true)?;

assert_eq!(validator.get("20100")?, Some(&true));  // CAP Milano
assert_eq!(validator.get("ABC2001")?, Some(&true)); // Codice prodotto
```

### Pattern Complessi

```rust
let mut complex: PatternTrie<String> = PatternTrie::default();

// Numero di telefono con area code opzionale
complex.add("[0-3]*[1-9][0-9][0-9]+", "telefono".to_string())?;

// ID con prefisso variabile e suffisso obbligatorio  
complex.add("[AB]*[1-9]+XXX", "id_speciale".to_string())?;
```

## API Reference

### `PatternTrie<T>`

#### Metodi Principali

- `PatternTrie::default()` - Crea un nuovo trie vuoto
- `add(&mut self, pattern: &str, value: T) -> Result<(), PatternTrieError>` - Inserisce un pattern
- `get(&self, input: &str) -> Result<Option<&T>, PatternTrieError>` - Cerca un valore

#### Vincoli sui Tipi

- `T` deve implementare `Default + Clone + Debug`

#### Errori

```rust
pub enum PatternTrieError {
    InvalidDigit(char),                    // Carattere non numerico
    UnexpectedChar(char),                  // Carattere non supportato nella sintassi
    MissingClosingBracket,                 // Parentesi non chiusa
    UnexpectedEmptyRange,                  // Parentesi vuote []
    PathAlreadyExists,                     // Percorso già esistente con valore diverso
}
```

## Architettura Interna

Il trie utilizza una architettura a nodi tipizzati:

- **Root**: Nodo radice del trie
- **Exact(digit)**: Nodo che corrisponde esattamente a un digit
- **Repeatable(digit)**: Nodo che può ripetere un digit zero o più volte

I pattern vengono compilati in sequenze di token e poi costruiti come percorsi nel trie, garantendo navigazione efficiente e gestione corretta delle ripetizioni.

## Gestione degli Errori

Tutti i metodi restituiscono `Result` per una gestione robusta degli errori:

```rust
match trie.add("12[a]3", "invalid".to_string()) {
    Ok(_) => println!("Pattern aggiunto"),
    Err(PatternTrieError::InvalidDigit('a')) => {
        println!("Errore: solo digit 0-9 sono permessi nelle parentesi");
    }
    Err(e) => println!("Altri errori: {}", e),
}
```

## Limitazioni

- **Conflitti di pattern**: Pattern che generano percorsi identici con valori diversi causano errore
- **Solo digit numerici**: Non supporta lettere o caratteri speciali
- **Memoria**: Pattern molto complessi possono generare molte varianti

## Complessità

- **Inserimento**: O(k × v) dove k è lunghezza pattern e v è numero di varianti generate
- **Ricerca**: O(k) dove k è la lunghezza dell'input
- **Spazio**: O(n × k) ottimizzato per prefissi condivisi

## Casi d'Uso

- **Routing telefonico**: Mappatura prefissi a operatori con pattern flessibili
- **Validazione input**: Controllo formato codici, ID, numeri
- **Parsing configurazioni**: Gestione pattern di configurazione numerica
- **Classificazione stringhe**: Categorizzazione basata su pattern numerici
- **Cache intelligente**: Chiavi con pattern per hit rate ottimizzato

## Test

```bash
# Esegui tutti i test
cargo test

# Test specifici
cargo test tokenizer
cargo test node  
cargo test map
```

Copertura dei test:
- **Tokenizer**: Parsing pattern, gestione errori sintassi
- **Node**: Architettura interna, navigazione, merge nodi
- **Map**: Funzionalità complete inserimento/ricerca, pattern complessi
- Casi limite e edge cases
- Performance con pattern complessi

## Licenza

Questo progetto è distribuito sotto licenza MIT. Vedi il file [LICENSE](LICENSE) per i dettagli.

## Contributi

I contributi sono benvenuti! Guidelines:

1. Fork del repository
2. Crea branch per la feature (`git checkout -b feature/nuova-funzionalita`)
3. Aggiungi test per le nuove funzionalità
4. Assicurati che tutti i test passino (`cargo test`)
5. Committa con messaggi descrittivi
6. Apri una Pull Request

## Roadmap

- [ ] Metodo per rimuovere pattern
- [ ] Supporto per range `[0-9]` nella sintassi
- [ ] Ottimizzazioni per pattern con molte varianti
- [ ] Serializzazione/deserializzazione dei trie
- [ ] Supporto per uso concorrente
- [ ] Metriche e profiling integrati

---

**Nota**: Progetto in sviluppo attivo. API può cambiare nelle versioni pre-1.0.
