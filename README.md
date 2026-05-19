# Material Symbols for Rust

## Info
This fork updates the original with the newer Material Symbols icon set from 2022. It also:
- Replaces .ttf files with .woff2
- Adds a few dependencies that make life easier. There's a `no-deps` branch that has no external dependencies but I'm not going to maintain it. 

## Feature flags
* `["sharp"]` *(Default)*
* `["outlined"]` 
* `["rounded"]`


## Things I'll probably add
- Pictogrammers MDI icons

---

This repo contains the auto-generated mapping from an icon name
(such as `Icon::NetworkWifi`) to the character codepoint `\u{e1ba}`)
in Google's Material Symbols variable font ([https://material.io/tools/icons/](https://material.io/tools/icons/))

## Example

```rust
use material_symbols::{Icon, char_from_icon};
let icon_char = char_from_icon(Icon::SharpOralDisease);
assert_eq!('\u{e116}', icon_char);
```

When the resulting character is displayed using any font renderer,
the character will result in the "oral_disease" glyph.

## License

According to [https://github.com/google/material-design-icons/blob/master/LICENSE](https://github.com/google/material-design-icons/blob/master/LICENSE)
(retrieved 23-10-2018) the font data embedded in this library is licensed under
the Apache 2.0 license, which explains the license for this crate.

You do not need to attribute Google (although they appreciate it):

> "We have made these icons available for you to incorporate into your products
> under the Apache License Version 2.0. Feel free to remix and re-share these
> icons and documentation in your products. We'd love attribution in your app's
> about screen, but it's not required. The only thing we ask is that you not
> re-sell these icons."
