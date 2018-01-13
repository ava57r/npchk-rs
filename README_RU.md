# npchk-rs

Библиотека для проверки статуса контрагентов через сервис [http://npchk.nalog.ru/](http://npchk.nalog.ru/).

## Установка

```rust
extern crate npchk;

```

## Пример использования

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