pub fn reverse(input: &str) -> String {
    let mut reversed: String = "".to_string();

    for _i in input.chars().rev() {
        reversed += &String::from(_i);
    }

    return reversed;
}
