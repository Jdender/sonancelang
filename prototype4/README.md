# Sonance

![GitHub](https://img.shields.io/github/license/Jdender/sonancelang)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Jdender/sonancelang/Continuous%20integration)

[_noun_](https://www.dictionary.com/browse/sonance)

1. the condition or quality of being sonant.
2. a sound; a tune.
3. an work in progress, experimental programing language.

# Iteration 3

This is the third iteration of the compiler so far.

## Example

```swift
declare "c" {
    func getchar() -> I32;
    func putchar(char: I32) -> I32;
}

public "c" func main() -> I32 {
    let char = getchar();
    if char != -1 {
        putchar(char);
        main();
    }
}
```

## Compile A Program

```bash
# Run the compiler with input and output
cargo run input.son output.o

# Use GCC to link to libc
gcc -o output output.o

# Run output program
./output
```

## Features

### Functions

```swift
func name(param: ParamType) -> ReturnType {
    ...
}
```

### Blocks

Final expression with no semicolon becomes return value.

When there is no return it defaults to a `0` with type `I32`.

```swift
{
    let t = do_thing(); // Use `let` to bind variables
    do_other(t, t);     // Pass args separated by `,`
    123
}
```

### Types

Used for parameters and return values.

```swift
I8 I16 I32 I64 // Signed integers
U8 U16 U32 U64 // Unsigned
F32 F64        // Floating point
ISize USize    // Pointer width
```

### Numbers / Math

```swift
123        // Defaults to `I32`
123.0      // Defaults to `F32`
123 as I64 // Use `as` to specify literal type

+ - * /
== != >= <= < >
```

### If Else

For any type, `0` is considered "false", and everything else is considered "true."

The `else` clause may be omitted; it will default to `0` of what ever type the `if` clause was.

```swift
if 12 > 0 {
    ...
} else {
    ...
}
```

### Function Declarations

Use to link to other symbols.

```swift
declare "c" {
    func c_func(param: I32) -> I32;
}
```

## Limitations

- No loops, you have to use recursion instead
- No booleans or boolean operators, use `!= 0` and `if` expressions
- No bitwise operators or dereference operator

## License

[MIT](https://choosealicense.com/licenses/mit/)
