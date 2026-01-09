// my print
pub struct MyPrint {
    value: Vec<String>,
}

#[allow(dead_code)]
impl MyPrint {
    // new a MyPrint
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }
    // get value
    pub fn get(&self) -> String {
        self.value.concat()
    }
    // get value, and destroy/drop my print, release my print
    pub fn flush(self) -> String {
        self.value.into_iter().collect()
    }
    // print a line
    pub fn print_line<S: Into<String>>(&mut self, s: S) {
        let mut line = s.into();
        line.push('\n');
        self.value.push(line);
    }
    // append
    pub fn print<S: Into<String>>(&mut self, s: S) {
        self.value.push(s.into());
    }
}

// my rust
pub fn my_rust(data_1: &str, data_2: &str) -> String {
    let mut my_print = MyPrint::new();
    my_print.print(data_1);
    my_print.print(" ");
    my_print.print(data_2);
    my_print.flush()
}
