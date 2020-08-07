# Sonance

![GitHub](https://img.shields.io/github/license/Jdender/sonancelang)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Jdender/sonancelang/Continuous%20integration)

[_noun_](https://www.dictionary.com/browse/sonance)

1. the condition or quality of being sonant.
2. a sound; a tune.
3. an work in progress, experimental programing language.

## Alpha Checklist

- [x] Int Literals
- [x] Math Operators
- [x] Local Variables
- [x] Block Scopes
- [x] If Else Statements
- [x] Functions and Calls
- [x] Function Arguments/Parameters
- [ ] External Function Calls/Defs
- [ ] Booleans & Boolean Operators
- [ ] Anonymous Tuples

## Example

```swift
export func sonance() -> I32 {
    fibonacci(30)
}

func fibonacci(num: I32) -> I32 {
    if num <= 1 {
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
```

## Inspiration

- Rust
- C
- TypeScript
- Swift

## License

[MIT](https://choosealicense.com/licenses/mit/)
