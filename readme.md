# LTPP-RUST-OUTPUT

[Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/)

## Description

- A Rust-based output library that supports various output functionalities through functions, constructors, etc., allowing customization of text and background colors.

## Features

- Supports formatted time output
- Supports code location output
- Allows customization of text color, background color, text boldness, and other configurations
- Supports defining structures for output information
- Supports defining constructors for output information

## Installation

```shell
cargo add ltpp-output
```

## Code Example

### Struct Output

#### Using the `output` Function

```rust
use ltpp_output::*;
output(Output {
    text: "test_output_struct",
    text_color: Some(ColorType::Use(Color::Default)),
    text_bg_color: Some(ColorType::Color256(0x000000)),
    show_time: Some(true),
    show_code_location: Some(true),
    time_text_color: Some(ColorType::Rgb(255, 255, 255)),
    time_bg_color: Some(ColorType::Use(Color::Yellow)),
    code_location_text_color: Some(ColorType::Color256(0xffffff)),
    code_location_bg_color: Some(ColorType::Use(Color::Yellow)),
    split: Some(" => "),
    split_color: Some(ColorType::Use(Color::Cyan)),
    split_bg_color: Some(ColorType::Use(Color::Yellow)),
    ..Default::default()
});
```

#### Using the `output` Method

```rust
use ltpp_output::*;
Output {
    text: "test_output_struct_output",
    text_color: Some(ColorType::Use(Color::Default)),
    text_bg_color: Some(ColorType::Color256(0x000000)),
    show_time: Some(true),
    show_code_location: Some(true),
    time_text_color: Some(ColorType::Rgb(255, 255, 255)),
    time_bg_color: Some(ColorType::Use(Color::Yellow)),
    code_location_text_color: Some(ColorType::Color256(0xffffff)),
    code_location_bg_color: Some(ColorType::Use(Color::Yellow)),
    split: Some(" => "),
    split_color: Some(ColorType::Use(Color::Cyan)),
    split_bg_color: Some(ColorType::Use(Color::Yellow)),
    ..Default::default()
}
.output();
```

### Constructor Output

#### Using the `output` Function

```rust
use ltpp_output::*;
output(
    OutputBuilder::new()
        .set_text("test_output_builder")
        .set_text_color(ColorType::Color256(0xffffff))
        .set_time_text_color(ColorType::Rgb(255, 200, 255))
        .set_code_location_text_color(ColorType::Use(Color::Yellow))
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .build()
);
```

#### Using the `output` Method

```rust
use ltpp_output::*;
OutputBuilder::new()
    .set_text("test_output_builder_output")
    .set_text("test_output_builder")
    .set_text_color(ColorType::Color256(0xffffff))
    .set_time_text_color(ColorType::Rgb(255, 200, 255))
    .set_code_location_text_color(ColorType::Use(Color::Yellow))
    .set_text_blod(true)
    .set_time_text_blod(true)
    .set_code_location_text_blod(true)
    .set_show_time(true)
    .set_show_code_location(true)
    .build()
    .output();
```

### Output Macro

#### Struct Input

```rust
use ltpp_output::*;
output_macro!(Output {
    text: "test_proc_macro",
    text_color: Some(ColorType::default()),
    text_bg_color: Some(ColorType::Use(Color::Yellow)),
    show_time: Some(true),
    show_code_location: Some(true),
    time_text_color: Some(ColorType::Use(Color::Green)),
    time_bg_color: Some(ColorType::Color256(0xffffff)),
    code_location_text_color: Some(ColorType::Use(Color::Blue)),
    code_location_bg_color: Some(ColorType::Rgb(255, 200, 255)),
    split: Some(" => "),
    split_color: Some(ColorType::Use(Color::Cyan)),
    split_bg_color: Some(ColorType::Use(Color::Yellow)),
    ..Default::default()
});
```

#### Constructor Input

```rust
use ltpp_output::*;
OutputBuilder::new()
    .set_text("test_output_builder")
    .set_text_color(ColorType::Color256(0xffffff))
    .set_time_text_color(ColorType::Rgb(255, 200, 255))
    .set_code_location_text_color(ColorType::Use(Color::Yellow))
    .set_text_blod(true)
    .set_time_text_blod(true)
    .set_code_location_text_blod(true)
    .set_show_time(true)
    .set_show_code_location(true)
    .build()
```

#### Multiple Inputs

```rust
use ltpp_output::*;
output_macro!(
    Output {
        text: "test_proc_macro",
        text_color: Some(ColorType::default()),
        text_bg_color: Some(ColorType::Use(Color::Yellow)),
        show_time: Some(true),
        show_code_location: Some(true),
        time_text_color: Some(ColorType::Use(Color::Green)),
        time_bg_color: Some(ColorType::Color256(0xffffff)),
        code_location_text_color: Some(ColorType::Use(Color::Blue)),
        code_location_bg_color: Some(ColorType::Rgb(255, 200, 255)),
        split: Some(" => "),
        split_color: Some(ColorType::Use(Color::Cyan)),
        split_bg_color: Some(ColorType::Use(Color::Yellow)),
        ..Default::default()
    },
    OutputBuilder::new()
        .set_text("test_output_builder")
        .set_text_color(ColorType::Color256(0xffffff))
        .set_time_text_color(ColorType::Rgb(255, 200, 255))
        .set_code_location_text_color(ColorType::Use(Color::Yellow))
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .build()
);
```

### Color Usage

- `ColorType::Use`: Use built-in colors
- `ColorType::Color256`: Hexadecimal
- `ColorType::Rgb`: RGB color (r, g, b)

#### ColorType::Use

```rust
ColorType::Use(Color::White)
```

#### ColorType::Color256

```rust
ColorType::Color256(0xffffff)
```

#### ColorType::Rgb

```rust
ColorType::Rgb(255,255,255)
```
