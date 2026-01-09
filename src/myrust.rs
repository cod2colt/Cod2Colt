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

// function list
pub const FUNCTION: [&str; 2] = ["Data", "Flow"];

// my rust
pub fn my_rust(function: &str, data: &str) -> String {
    let mut my_print = MyPrint::new();

    // show the title
    for _i in 0..99 {
        my_print.print('=');
    }
    my_print.print_line("\nMy Rust Course");
    for _i in 0..99 {
        my_print.print('=');
    }

    // parser function
    match function {
        // Function: Data
        f if f == FUNCTION[0] => {
            my_print.print_line("=== My Data ===");
            my_data(&mut my_print);
        }
        // Function: Flow
        f if f == FUNCTION[1] => {
            my_print.print_line("=== My Flow ===");
            my_flow(&mut my_print);
        }
        // default
        _ => {
            my_print.print(function);
            my_print.print(" ");
            my_print.print(data);
        }
    }
    my_print.flush()
}

// data type
fn my_data(print_out: &mut MyPrint) {
    // study and test data type
    print_out.print_line("=== int data type ===");
    let d1: i32 = 13;
    let mut d2: i32 = 24;
    let mut res = d1 + d2; // mut for mutable
    let str_output = format!("{} + {} = {}", d1, d2, res);
    print_out.print_line(str_output);
    d2 = 3;
    res = d1 / d2;
    let str_output = format!("{} / (int32) {} = {}", d1, d2, res);
    print_out.print_line(str_output);
    let d2 = 3.0;
    let res = d1 as f32 / d2;
    let str_output = format!("{} / (f32) {:.2} = {}", d1, d2, res);
    print_out.print_line(str_output);
    // array
    let my_array = [7, 6, 5, 4, 3, 2, 1, 0];
    for i in 0..=7 {
        print_out.print(format!("{} ", my_array[i]));
    }
    // tuple
    let my_tuple = (5, "hello", 3, '\n');
    print_out.print_line("\n=== Tuple ===");
    print_out.print(format!("{} ", my_tuple.0));
    print_out.print(format!("{} ", my_tuple.1));
    print_out.print(format!("{} ", my_tuple.2));
    print_out.print(format!("{} ", my_tuple.3));
}

// control flow
fn my_flow(print_out: &mut MyPrint) {
    // loop
    print_out.print_line("=== loop ===");
    let mut i = 0;
    loop {
        i += 1;
        print_out.print_line(format!("loop: {}", i));
        if i > 8 {
            break;
        }
    }

    // while
    print_out.print_line("=== while ===");
    i = 0;
    while i < 8 {
        i += 1;
        print_out.print_line(format!("while: {}", i));
    }

    // for
    print_out.print_line("=== for ===");
    for i in 0..8 {
        print_out.print_line(format!("while: {}", i));
    }
}
