extern crate chrono;
extern crate npchk;

use npchk::*;
use chrono::prelude::*;

fn main() {
    match check_fns_partner(Partner::new("4205036750", "420501001", Utc::now())) {
        Ok(rsp) => println!("{:?}", rsp),
        Err(e) => println!("Error {:?}", e),
    }
}
