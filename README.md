# Sonance

![GitHub](https://img.shields.io/github/license/Jdender/sonancelang)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Jdender/sonancelang/Continuous%20integration)

[_noun_](https://www.dictionary.com/browse/sonance)

1. the condition or quality of being sonant.
2. a sound; a tune.
3. an work in progress, experimental programing language.

## Alpha Roadmap

- [ ] Int Literals
- [ ] Math and Boolean Operators
- [ ] Local Variables
- [ ] Block Scopes
- [ ] If Else Statements
- [ ] Functions and Calls
- [ ] Anonymous Tuples

## Inspiration

- Rust
- C
- TypeScript
- Swift

## Future Example

```swift
func main() {
    say_hello(_);
    for i in 0..=100 {
        say_hello(fizzbuzz(i));
    }
}

func say_hello(name: String = "world") {
    print("Hello, \(name) !");
}

func fizzbuzz(num: U32) -> StringBuf {
    let tuple = struct (num % 3, num % 5);

    match tuple {
        (0, 0) -> "Fizzbuzz",
        (0, _) -> "Fizz",
        (_, 0) -> "Buzz",
        _ -> num.to_string(),
    }
}
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
