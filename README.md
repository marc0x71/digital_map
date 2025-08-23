# Digital Map

Una struttura dati efficiente in Rust per mappare stringhe numeriche a valori, implementata come trie digitale.

## Descrizione

`Digital Map` è un trie (albero di prefissi) specializzato per stringhe contenenti solo cifre numeriche (0-9). Questa struttura dati permette di inserire e recuperare valori associati a chiavi numeriche in modo efficiente, con complessità temporale O(k) dove k è la lunghezza della chiave.

## Caratteristiche

- **Efficiente**: Inserimento e ricerca in tempo O(k)
- **Flessibile**: Supporta qualsiasi tipo di valore che implementi `Default`
- **Sicuro**: Gestisce solo stringhe numeriche valide
- **Memory-efficient**: Condivide percorsi comuni tra chiavi simili

## Utilizzo

```rust
use digital_map::Map;

fn main() {
    let mut map: Map<String> = Map::default();
    
    // Inserimento di valori
    map.add("123", "centoventitre".to_string());
    map.add("456", "quattrocinquantasei".to_string());
    map.add("12", "dodici".to_string());
    
    // Recupero di valori
    println!("{:?}", map.get("123")); // Some("centoventitre")
    println!("{:?}", map.get("456")); // Some("quattrocinquantasei")
    println!("{:?}", map.get("789")); // None
    
    // Supporto per chiavi sovrapposte
    println!("{:?}", map.get("12"));  // Some("dodici")
    println!("{:?}", map.get("123")); // Some("centoventitre")
}
```

## Esempi Avanzati

### Diversi Tipi di Valore

```rust
// Map con valori interi
let mut int_map: Map<i32> = Map::default();
int_map.add("42", 100);
int_map.add("123", 200);

// Map con strutture complesse
let mut vec_map: Map<Vec<String>> = Map::default();
vec_map.add("001", vec!["primo".to_string(), "elemento".to_string()]);
```

### Gestione di Prefissi

```rust
let mut map: Map<String> = Map::default();

// Chiavi che condividono prefissi
map.add("12", "dodici".to_string());
map.add("123", "centoventitre".to_string());
map.add("1234", "millerduecentotrentaquattro".to_string());

// Ogni chiave è indipendente
assert_eq!(map.get("12"), Some(&"dodici".to_string()));
assert_eq!(map.get("123"), Some(&"centoventitre".to_string()));
assert_eq!(map.get("1"), None); // Non esiste se non aggiunta esplicitamente
```

## API Reference

### `Map<T>`

#### Metodi

- `Map::default()` - Crea una nuova mappa vuota
- `add(&mut self, input: &str, value: T)` - Inserisce un valore associato alla chiave numerica
- `get(&self, input: &str) -> Option<&T>` - Recupera il valore associato alla chiave

#### Vincoli

- `T` deve implementare `Default`
- Le chiavi devono contenere solo caratteri numerici (0-9)

### `Node<T>`

Struttura interna che rappresenta un nodo nel trie. Generalmente non utilizzata direttamente.

## Gestione degli Errori

La struttura genera panic nei seguenti casi:

- **Input non valido**: Se una stringa contiene caratteri non numerici
  ```rust
  map.add("12a3", value); // Panic: "input must contain only digits"
  map.get("xyz");         // Panic: "input must contain only digits"
  ```

## Complessità

- **Inserimento**: O(k) dove k è la lunghezza della chiave
- **Ricerca**: O(k) dove k è la lunghezza della chiave
- **Spazio**: O(n*k) nel caso peggiore, ma ottimizzato per prefissi condivisi

## Casi d'Uso

- **Routing telefonico**: Mappatura di prefissi telefonici a operatori
- **Codici postali**: Associazione di CAP a informazioni geografiche
- **Identificatori numerici**: Qualsiasi sistema che richieda mappature efficienti di ID numerici
- **Cache con chiavi numeriche**: Storing e retrieval rapido basato su identificatori numerici

## Test

Il progetto include una suite completa di test:

```bash
cargo test
```

I test coprono:
- Funzionalità di base (inserimento/ricerca)
- Gestione di chiavi sovrapposte
- Diversi tipi di valore
- Casi limite e gestione errori
- Strutture ad albero complesse

## Licenza

Questo progetto è distribuito sotto licenza MIT. Vedi il file [LICENSE](LICENSE) per i dettagli.

## Contributi

I contributi sono benvenuti! Per favore:

1. Fai un fork del repository
2. Crea un branch per la tua feature (`git checkout -b feature/nuova-funzionalita`)
3. Committa le tue modifiche (`git commit -am 'Aggiunge nuova funzionalità'`)
4. Pusha sul branch (`git push origin feature/nuova-funzionalita`)
5. Apri una Pull Request

## Roadmap

- [ ] Metodo per rimuovere chiavi
- [ ] Gestione degli errori con `Result` invece di panic
- [ ] Ottimizzazioni per uso concorrente
- [ ] Supporto per pattern matching sui prefissi

---

**Nota**: Questo è un progetto educativo/sperimentale. Per uso in produzione, considera le tue specifiche esigenze di performance e sicurezza.
