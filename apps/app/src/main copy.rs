extern crate rutie;

use rutie::{Object, RString, VM, Fixnum, Class, Hash, Symbol};

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
    VM::init_loadpath();

    let result = VM::eval("2+3").ok().unwrap().try_convert_to::<Fixnum>();
    println!("{}", result.unwrap().to_i64());

    let result = VM::eval("'aaa' + 'bbb'").ok().unwrap().try_convert_to::<RString>();
    println!("{:?}", result.unwrap().to_string());

    let src = r#"
        require 'erb'
        str = "erb"
        erb = ERB.new('はじめての <%= str.upcase %>の実行')
        erb.result(binding)
    "#;
    let result = VM::eval(src).ok().unwrap().try_convert_to::<RString>();
    println!("{}", result.unwrap().to_string());

   let data = "rust";

   let src = format!(r#"
        require 'erb'
        str = "erb"
        erb = ERB.new('はじめての <%= "{}".upcase %>の実行')
        erb.result(binding)
    "#, data);
    let result = VM::eval(&src).ok().unwrap().try_convert_to::<RString>();
    println!("{}", result.unwrap().to_string());

    VM::require("erb");
    let src = RString::new_utf8(r#"
        はじめての <%= data.upcase %>の実行
    "#);
    let arguments = [src.to_any_object()];
    let erb = Class::from_existing("ERB").new_instance(&arguments);
    let mut hash = Hash::new();
    hash.store(Symbol::new("data"), RString::new_utf8("ruby"));

    let result = unsafe { erb.send("result_with_hash", &[hash.to_any_object()]) };
    println!("{}", result.try_convert_to::<RString>().unwrap().to_string());

    println!("{}", try_it("予定表～①💖ﾊﾝｶｸだ"));

}