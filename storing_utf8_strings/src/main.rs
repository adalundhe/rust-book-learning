fn main() {
    // Recall we can create a string as below:

    let mut s = String::new();
    println!("Begin here with an empty string - {}", s);
    
    // We can specify a type str here.
    let data = "initial contents";

    // And then convert it to type String using the
    // .to_string() method.
    s = data.to_string();

    // Voila!
    println!("Initial string - {}", s);

    // String types are UTF-8 encoded, so we can also store the following.

    s = String::from("السلام عليكم");
    println!("String is now - {}\n", s);

    s = String::from("Dobrý den");
    println!("String is now - {}\n", s);

    s = String::from("Hello");
    println!("String is now - {}\n", s);

    s = String::from("שָׁלוֹם");
    println!("String is now - {}\n", s);

    s = String::from("नमस्ते");
    println!("String is now - {}\n", s);

    s = String::from("こんにちは");
    println!("String is now - {}\n", s);

    s = String::from("안녕하세요");
    println!("String is now - {}\n", s);

    s = String::from("你好");
    println!("String is now - {}\n", s);

    s = String::from("Olá");
    println!("String is now - {}\n", s);

    s = String::from("Здравствуйте");
    println!("String is now - {}\n", s);

    s = String::from("Hola");
    println!("String is now - {}\n", s);

    // We can append one String to another String via the
    // .push_str() method as below.
    s = String::from("Hello, ");
    s.push_str("world");
    println!("String is now - {}\n", s);

    // We can also append a single character to a String
    // via the .push() method as below:
    s.push('!');
    // Note that, like C/C++, chars require single quotes.

    // We can also use `+` as the concatenation operator
    // between two strings OR we can use the format!()
    // macro.   
    s = String::from("Hello again, ");
    s += &String::from("world!");
    println!("String is now - {}\n", s);

    // The format!() macro is preferable if we need
    // to do more than simple string concatenation.
    s = format!("{}, {}!", "Hello", "Rust");
    println!("String is now - {}\n", s);

    // Unlike in Python and C/C++, strings are NOT indexable. So:
    //
    //    let hello = String::from("hello");
    //
    //    let text = hello[0];
    //
    // Won't work, and neither will:
    //
    //    let text = &hello[0];
    //
    // This is because String is a wrapper for `Vec<u8>` where
    // `u8` references a UTF-8 char/byte encoding (i.e. 3-208);
    // Rust doesn't allow indexing ude to this and not wanting to
    // walk through an entire vector to validate characters (when)
    // indexing is supposed to be O(1) time.

    // Rust strings *do* support slicing, but it is *not* recommended.
    let hello = "Test";
    let text = &hello[0..1];
    println!("Got - {} - as char at slice 0..1\n", text);

    // Instead Rust recommends using the .chars() method and iterating over
    // the result:
    for c in hello.chars() {
        println!("Char is - {}", c);
    }

    // We can also iterate over the bytes values via the .bytes()
    // method:
    for b in hello.bytes() {
        println!("Byte is - {}", b);
    }
}
