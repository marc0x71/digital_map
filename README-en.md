# Pattern Trie

An efficient Rust data structure for mapping numeric patterns to values, implemented as a digital trie with support for simple regular expressions.

## Description

`Pattern Trie` is a specialized trie (prefix tree) for numeric patterns that supports regex-like syntax for numeric digits (0-9). This data structure allows you to define flexible patterns and retrieve associated values efficiently, with O(k) time complexity where k is the input length.

## Features

- **Flexible patterns**: Supports variants `[34]`, repetitions `[3]*` and mandatory `[3]+`
- **Efficient**: Insertion and search in O(k) time
- **Type-safe**: Supports any value type implementing `Default + Clone + Debug`
- **Robust error handling**: Complete validation with informative messages
- **Memory-efficient**: Shares common paths and handles overlapping patterns

## Supported Syntax

- **Single digits**: `123` → matches exactly "123"
- **Variants**: `12[34]5` → matches "1235" or "1245"  
- **Zero or more**: `12[3]*4` → matches "124", "1234", "12334", etc.
- **At least one**: `12[3]+4` → matches "1234", "12334", etc. (but not "124")
- **Combinations**: `1[23]*[45]+6` → complex patterns with multiple rules

## Usage

```rust
use pattern_trie::PatternTrie;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut trie: PatternTrie<String> = PatternTrie::default();
    
    // Simple patterns
    trie.add("123", "fixed number".to_string())?;
    
    // Patterns with variants
    trie.add("12[34]5", "central variants".to_string())?;
    
    // Patterns with repetitions
    trie.add("1[2]*3", "zero or more 2s".to_string())?;
    trie.add("1[4]+5", "at least one 4".to_string())?;
    
    // Search
    println!("{:?}", trie.get("123")?);     // Some("fixed number")
    println!("{:?}", trie.get("1235")?);    // Some("central variants")
    println!("{:?}", trie.get("1245")?);    // Some("central variants")
    println!("{:?}", trie.get("13")?);      // Some("zero or more 2s")
    println!("{:?}", trie.get("1223")?);    // Some("zero or more 2s")
    println!("{:?}", trie.get("145")?);     // Some("at least one 4")
    println!("{:?}", trie.get("15")?);      // None (missing mandatory 4)
    
    Ok(())
}
```

## Advanced Examples

### Telephone Routing

```rust
let mut routing: PatternTrie<String> = PatternTrie::default();

// National prefixes
routing.add("39[0-9]*", "Italy".to_string())?;
routing.add("1[2-9]*", "North America".to_string())?;
routing.add("44[0-9]*", "United Kingdom".to_string())?;

// Emergency numbers
routing.add("11[28]", "Emergency".to_string())?;

assert_eq!(routing.get("393451234567")?, Some(&"Italy".to_string()));
assert_eq!(routing.get("112")?, Some(&"Emergency".to_string()));
```

### Code Validation

```rust
let mut validator: PatternTrie<bool> = PatternTrie::default();

// Italian postal codes
validator.add("[0-9][0-9][0-9][0-9][0-9]", true)?;

// Product codes with specific patterns
validator.add("ABC[1-3]+[0-9]*", true)?;

assert_eq!(validator.get("20100")?, Some(&true));  // Milan ZIP
assert_eq!(validator.get("ABC2001")?, Some(&true)); // Product code
```

### Complex Patterns

```rust
let mut complex: PatternTrie<String> = PatternTrie::default();

// Phone number with optional area code
complex.add("[0-3]*[1-9][0-9][0-9]+", "phone".to_string())?;

// ID with variable prefix and mandatory suffix  
complex.add("[AB]*[1-9]+XXX", "special_id".to_string())?;
```

## API Reference

### `PatternTrie<T>`

#### Main Methods

- `PatternTrie::default()` - Creates a new empty trie
- `add(&mut self, pattern: &str, value: T) -> Result<(), PatternTrieError>` - Inserts a pattern
- `get(&self, input: &str) -> Result<Option<&T>, PatternTrieError>` - Searches for a value

#### Type Constraints

- `T` must implement `Default + Clone + Debug`

#### Errors

```rust
pub enum PatternTrieError {
    InvalidDigit(char),                    // Non-numeric character
    UnexpectedChar(char),                  // Unsupported character in syntax
    MissingClosingBracket,                 // Unclosed bracket
    UnexpectedEmptyRange,                  // Empty brackets []
    PathAlreadyExists,                     // Existing path with different value
}
```

## Internal Architecture

The trie uses a typed node architecture:

- **Root**: Root node of the trie
- **Exact(digit)**: Node that matches exactly one digit
- **Repeatable(digit)**: Node that can repeat a digit zero or more times

Patterns are compiled into token sequences and then built as paths in the trie, ensuring efficient navigation and correct repetition handling.

## Error Handling

All methods return `Result` for robust error handling:

```rust
match trie.add("12[a]3", "invalid".to_string()) {
    Ok(_) => println!("Pattern added"),
    Err(PatternTrieError::InvalidDigit('a')) => {
        println!("Error: only digits 0-9 are allowed in brackets");
    }
    Err(e) => println!("Other errors: {}", e),
}
```

## Limitations

- **Pattern conflicts**: Patterns generating identical paths with different values cause errors
- **Numeric digits only**: Does not support letters or special characters
- **Memory**: Very complex patterns may generate many variants

## Complexity

- **Insertion**: O(k × v) where k is pattern length and v is number of generated variants
- **Search**: O(k) where k is input length
- **Space**: O(n × k) optimized for shared prefixes

## Use Cases

- **Telephone routing**: Prefix mapping to operators with flexible patterns
- **Input validation**: Format checking for codes, IDs, numbers
- **Configuration parsing**: Numeric configuration pattern handling
- **String classification**: Categorization based on numeric patterns
- **Smart caching**: Pattern-based keys for optimized hit rates

## Testing

```bash
# Run all tests
cargo test

# Specific tests
cargo test tokenizer
cargo test node  
cargo test map
```

Test coverage:
- **Tokenizer**: Pattern parsing, syntax error handling
- **Node**: Internal architecture, navigation, node merging
- **Map**: Complete insertion/search functionality, complex patterns
- Edge cases and boundary conditions
- Performance with complex patterns

## License

This project is distributed under the MIT license. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Guidelines:

1. Fork the repository
2. Create feature branch (`git checkout -b feature/new-functionality`)
3. Add tests for new features
4. Ensure all tests pass (`cargo test`)
5. Commit with descriptive messages
6. Open a Pull Request

## Roadmap

- [ ] Method to remove patterns
- [ ] Support for ranges `[0-9]` in syntax
- [ ] Optimizations for patterns with many variants
- [ ] Trie serialization/deserialization
- [ ] Support for concurrent usage
- [ ] Integrated metrics and profiling

---

**Note**: Project under active development. API may change in pre-1.0 versions.
