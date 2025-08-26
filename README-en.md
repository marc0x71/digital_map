# Digital Map

An efficient Rust data structure for mapping numeric strings to values, implemented as a digital trie.

## Description

`Digital Map` is a trie (prefix tree) specialized for strings containing only numeric digits (0-9). This data structure allows efficient insertion and retrieval of values associated with numeric keys, with O(k) time complexity where k is the key length.

## Features

- **Efficient**: Insert and search in O(k) time
- **Flexible**: Supports any value type that implements `Default`
- **Safe**: Handles only valid numeric strings
- **Memory-efficient**: Shares common paths between similar keys

## Usage

```rust
use digital_map::Map;

fn main() {
    let mut map: Map<String> = Map::default();
    
    // Insert values
    map.add("123", "one-hundred-twenty-three".to_string());
    map.add("456", "four-hundred-fifty-six".to_string());
    map.add("12", "twelve".to_string());
    
    // Retrieve values
    println!("{:?}", map.get("123")); // Some("one-hundred-twenty-three")
    println!("{:?}", map.get("456")); // Some("four-hundred-fifty-six")
    println!("{:?}", map.get("789")); // None
    
    // Support for overlapping keys
    println!("{:?}", map.get("12"));  // Some("twelve")
    println!("{:?}", map.get("123")); // Some("one-hundred-twenty-three")
}
```

## Advanced Examples

### Different Value Types

```rust
// Map with integer values
let mut int_map: Map<i32> = Map::default();
int_map.add("42", 100);
int_map.add("123", 200);

// Map with complex structures
let mut vec_map: Map<Vec<String>> = Map::default();
vec_map.add("001", vec!["first".to_string(), "element".to_string()]);
```

### Prefix Handling

```rust
let mut map: Map<String> = Map::default();

// Keys that share prefixes
map.add("12", "twelve".to_string());
map.add("123", "one-hundred-twenty-three".to_string());
map.add("1234", "one-thousand-two-hundred-thirty-four".to_string());

// Each key is independent
assert_eq!(map.get("12"), Some(&"twelve".to_string()));
assert_eq!(map.get("123"), Some(&"one-hundred-twenty-three".to_string()));
assert_eq!(map.get("1"), None); // Doesn't exist unless explicitly added
```

## API Reference

### `Map<T>`

#### Methods

- `Map::default()` - Creates a new empty map
- `add(&mut self, input: &str, value: T)` - Inserts a value associated with the numeric key
- `get(&self, input: &str) -> Option<&T>` - Retrieves the value associated with the key

#### Constraints

- `T` must implement `Default`
- Keys must contain only numeric characters (0-9)

### `Node<T>`

Internal structure representing a node in the trie. Generally not used directly.

## Error Handling

The structure panics in the following cases:

- **Invalid input**: If a string contains non-numeric characters
  ```rust
  map.add("12a3", value); // Panic: "input must contain only digits"
  map.get("xyz");         // Panic: "input must contain only digits"
  ```

## Complexity

- **Insertion**: O(k) where k is the key length
- **Search**: O(k) where k is the key length
- **Space**: O(n*k) in the worst case, but optimized for shared prefixes

## Use Cases

- **Telephone routing**: Mapping phone prefixes to operators
- **Postal codes**: Associating ZIP codes with geographic information
- **Numeric identifiers**: Any system requiring efficient mappings of numeric IDs
- **Numeric key caching**: Fast storing and retrieval based on numeric identifiers

## Testing

The project includes a comprehensive test suite:

```bash
cargo test
```

Tests cover:
- Basic functionality (insert/search)
- Overlapping key handling
- Different value types
- Edge cases and error handling
- Complex tree structures

## License

This project is distributed under the MIT license. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a branch for your feature (`git checkout -b feature/new-functionality`)
3. Commit your changes (`git commit -am 'Add new functionality'`)
4. Push to the branch (`git push origin feature/new-functionality`)
5. Open a Pull Request

## Roadmap

- [ ] Method to remove keys
- [X] Error handling with `Result` instead of panic
- [ ] Optimizations for concurrent use
- [ ] Support for prefix pattern matching

---

**Note**: This is an educational/experimental project. For production use, consider your specific performance and security requirements.
