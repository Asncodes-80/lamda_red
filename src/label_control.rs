fn label_control() {
    let v: String = String::from("Test this after you got it work. This message is very long to demonstrate it proper in proper screen or any object without any size overflow (width overflow).");
    let mut t: Vec<&str> = v.split_whitespace().collect();

    let mut edited_v: String = String::from("");

    let per_word = 8;

    if t.len() % per_word != 0 {
        for _ in 0..t.len() % per_word {
            t.push("");
        }
    }

    for i in 0..t.len() {
        if i % per_word == 0 {
            for j in i..i + per_word {
                edited_v.push_str(t[j]);
                edited_v.push_str(" ");
            }
            edited_v.push_str("\n");
        }
    }

    println!("{}", edited_v.trim());
}
