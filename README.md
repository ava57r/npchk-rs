# npchk-rs

Library to check the status of contractors through service [http://npchk.nalog.ru/](http://npchk.nalog.ru/).

## Installation

```rust
extern crate npchk;

```

## Example usage

```rust
extern crate chrono;
extern crate npchk;

use npchk::*;
use chrono::prelude::*;

fn main() {
    let mut partners: Vec<Partner> = vec![];
    partners.push(Partner::new("4205036750", "420501001", Utc::now()));
    partners.push(Partner::new("6648185610", "662301001", Utc::now()));

    match check_fns(partners) {
        Ok(rsp) => println!("{:?}", rsp),
        Err(e) => println!("Error {:?}", e),
    }
}
```

[README_RU.md](README_RU.md)