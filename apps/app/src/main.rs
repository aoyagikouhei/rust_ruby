extern crate rutie;

use rutie::{Object, RString, VM};

fn try_it(s: &str) -> String {
    let a = RString::new_utf8(s);

    // The `send` method returns an AnyObject type.
    let b = unsafe { a.send("reverse", &[]) };

    // We must try to convert the AnyObject
    // type back to our usable type.
    match b.try_convert_to::<RString>() {
        Ok(ruby_string) => ruby_string.to_string(),
        Err(_) => "Fail!".to_string(),
    }
}

fn main() {
    VM::init();

    println!("{}", try_it("予定表～①💖ﾊﾝｶｸだ"));
}