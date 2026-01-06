// my rust
pub fn my_rust(data_1: &str, data_2: &str) -> String {
    let mut str_output = String::from("");
    str_output.push_str(data_1);
    str_output.push_str(" ");
    str_output.push_str(data_2);
    str_output
}
