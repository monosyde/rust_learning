

fn print_struct_bytes(text: &String, start: usize) {
    let str_bytes = &text.as_bytes()[start-20..start+100];
    let cut_str = String::from_utf8(str_bytes.to_vec()).unwrap();
    println!("cut_str {:?}", cut_str);
}