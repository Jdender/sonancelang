# Sonance

[_noun_](https://www.dictionary.com/browse/sonance)

1. the condition or quality of being sonant.
2. a sound; a tune.
3. an work in progress, experimental programing language.

The compiler itself is written in Rust which is also the language most inspiration was taken from.Some ideas were also borrowed from TypeScript as well as syntactically from Swift.

## Alpha Roadmap

- [x] Int Literals
- [x] Math and Boolean Operators
- [x] Local Variables
- [x] Block Scopes
- [x] If Else Statements
- [ ] Functions and Calls
- [ ] Anonymous Tuples

## Future Examples

Most of the following features haven't been implemented at the time of writing this, take the following examples with a grain of salt.

### Hello world

```swift
func say_hello(name: String = "world") {
    print("Hello, #[name] !");
}
```

### Fizzbuzz

```swift
func main() {
    for i in 0..=100 {
        print(fizzbuzz(i));
    }
}

func fizzbuzz(num: U32) -> StringBuf {
    match struct(num % 3, num % 5) {
        (0, 0) -> "Fizzbuzz",
        (0, _) -> "Fizz",
        (_, 0) -> "Buzz",
        _ -> num.to_string(),
    }
}
```

### Cat Clone

The actual APIs will most likely be different, take the following as just an example.

```swift
import std::{
    future -> await_all,
    env -> { args, pwd },
    fs -> { read_file, path::join },
};

func main() -> Future[Void] {
    let files = args
        .split(" ")
        .map(func (file) {
            pwd()
                .join(file)
                .read_file()
        })
        .await_all()
        .use;

    for result in files {
        match result {
            Okay: file -> print(file.content),
            Error: reason -> {
                print("An error occurred: #[reason]");
                return;
            }
        }
    }
}
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
