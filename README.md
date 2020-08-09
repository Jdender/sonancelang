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
export func main() -> I32 {
    print_num(fibonacci(30));
    putchar(10); // Newline
}

func fibonacci(num: I32) -> I32 {
    if num <= 1 {
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}

declare "c" {
    func putchar(char: I32) -> I32;
}

func print_num(num: I32) -> I32 {
    if num > 9  {
        let next = num / 10;
        num = num - (10 * next);
        print_num(next);
    } else {}
    putchar(48 + num);
}
```

## Inspiration

- Rust
- C
- TypeScript
- Swift

## License

[MIT](https://choosealicense.com/licenses/mit/)
