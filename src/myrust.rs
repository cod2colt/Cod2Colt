// my print
pub struct MyPrint {
    input: Vec<String>,
    output: Vec<String>,
}

#[allow(dead_code)]
impl MyPrint {
    // new a MyPrint
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
        }
    }
    // get value
    pub fn get(&self) -> String {
        self.output.concat()
    }
    // get value, and destroy/drop my print, release my print
    pub fn flush(self) -> String {
        self.output.into_iter().collect()
    }
    // reset my print
    pub fn reset(&mut self) {
        self.input.clear();
        self.output.clear();
    }
    // print a line
    pub fn print_line<S: Into<String>>(&mut self, s: S) {
        let mut line = s.into();
        line.push('\n');
        self.output.push(line);
    }
    // append
    pub fn print<S: Into<String>>(&mut self, s: S) {
        self.output.push(s.into());
    }
}

// my function structure
pub struct MyFunction {
    // function name
    pub name: &'static str,
    // function pointer
    pub func: fn(&mut MyPrint),
}

// my test function
pub const MY_TEST_FUN: [MyFunction; 3] = [
    MyFunction {
        name: "Clear",
        func: my_clear,
    },
    MyFunction {
        name: "Data",
        func: my_data,
    },
    MyFunction {
        name: "Flow",
        func: my_flow,
    },
];

// my rust
pub fn my_rust(function: &str, data: &str) -> String {
    let mut my_print = MyPrint::new();
    // set the input data
    my_print.input.push(data.into());

    // show the title
    for _i in 0..120 {
        my_print.print('=');
    }
    my_print.print_line("\nMy Rust Course");
    for _i in 0..120 {
        my_print.print('=');
    }
    my_print.print('\n');

    // parser function
    if let Some(cmd) = MY_TEST_FUN.iter().find(|c| c.name == function) {
        (cmd.func)(&mut my_print);
    } else if ["ferris", "rust"].contains(&function) {
        my_print.input.push(function.into());
        my_ferris(&mut my_print);
    } else {
        my_print.input.push("Rust".into());
        my_ferris(&mut my_print);
        // default
        my_print.print(format!("<{}>", function));
        my_print.print(" ");
        my_print.print(format!("<{}>", data));
    }
    my_print.flush()
}

// hi, rust and ferris
fn my_ferris(print_out: &mut MyPrint) {
    // get func and name
    let mut func: String = print_out.input.pop().unwrap_or("Rustaceans".to_string());
    if func.is_empty() {
        func = "Rust".to_string();
    }
    let mut name: String = print_out.input.pop().unwrap_or("Rustaceans".to_string());
    if name.is_empty() {
        name = "Rustaceans".to_string();
    }

    let ferris = r#"
                _~^~^~_
            \) /  o o  \ (/
              '_   -   _'
              / '-----' \
    "#;
    print_out.print('\n');
    print_out.print_line(format!("         < Hello, {} and {}! >", func, name));
    print_out.print_line("                \\");
    print_out.print("                 \\");
    print_out.print_line(format!("{}", ferris));
}

// clear output
fn my_clear(print_out: &mut MyPrint) {
    print_out.reset();
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
    print_out.print_line("\nDestruct tuple");
    let (a, b, c, d) = my_tuple;
    print_out.print_line(format!("{} {} {} {}", a, b, c, d));

    // heap
    print_out.print_line("======== Heap ========");
    // vec
    {
        // vec block
        print_out.print_line("=== Vec ===");
        print_out.print_line("👉 for i");
        let mut v = vec![11, 22, 33, 44, 55];
        for i in 0..v.len() {
            print_out.print(format!("{}:{} ", i, v[i]));
        }
        print_out.print_line("\n👉 for get");
        for i in 0..=v.len() {
            if let Some(d) = v.get(i) {
                print_out.print(format!("{}:{} ", i, d));
            } else {
                print_out.print(format!("{}:outbound! ", i));
            }
        }
        print_out.print_line("\n👉 for in vec");
        for i in &v {
            print_out.print(format!("{} ", i));
        }
        print_out.print_line("\n👉 iter in vec");
        let s = &v
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        print_out.print_line(s);
        print_out.print_line("👉 push and pop vec");
        v.push(66);
        v.push(77);
        for i in &v {
            print_out.print(format!("{} ", i));
        }
        print_out.print('\n');
        for i in 0..=v.len() {
            let d = v.pop().unwrap_or(-1);
            print_out.print(format!("pop {}:{} ", i, d));
        }
    }

    // String
    print_out.print_line("\n=== String ===");

    {
        // String block
        let mut s = String::from("Hello");
        print_out.print_line(format!("Initial string: {}", s));

        // push char
        s.push(' ');
        s.push('w');
        s.push('o');
        s.push('r');
        s.push('l');
        s.push('d');
        print_out.print_line(format!("After push chars: {}", s));

        // push str
        s.push_str("!!!");
        print_out.print_line(format!("After push str: {}", s));

        // chars iter
        print_out.print_line(format!("Chars:"));
        for c in s.chars() {
            print_out.print(format!("{} ", c));
        }
        print_out.print('\n');

        // iter bytes
        print_out.print_line(format!("Bytes:"));
        for b in s.bytes() {
            print_out.print(format!("{} ", b));
        }
        print_out.print('\n');

        // into Vec<u8>
        let v: Vec<u8> = s.clone().into_bytes();
        print_out.print_line(format!("Vec<u8>: {:?}", v));

        // append String
        let s2 = String::from(" Rust");
        s.push_str(&s2); // push_str &str
        print_out.print_line(format!("After append s2: {}", s));

        // pop / truncate / clear
        s.pop();
        print_out.print_line(format!("After pop: {}", s));

        s.truncate(5);
        print_out.print_line(format!("After truncate(5): {}", s));

        s.clear();
        print_out.print_line(format!("After clear: {}", s));
    }

    // Box
    print_out.print_line("\n=== Box List ===");
    {
        // Box block
        enum MyBoxList {
            ListNode(i32, Box<MyBoxList>),
            End,
        }
        use MyBoxList::{End, ListNode};

        let mut my_list = ListNode(0, Box::new(End));

        // add a node
        let current: &mut MyBoxList = &mut my_list;
        let a_node = ListNode(1, Box::new(End));
        if let ListNode(_, next) = current {
            *next = Box::new(a_node);
        }

        // append b node
        let b_node = ListNode(2, Box::new(End));
        let mut current = &mut my_list;

        while let ListNode(_, next) = current {
            current = next;
        }
        *current = b_node;

        // append c node
        let c_node = ListNode(3, Box::new(End));
        if let ListNode(_, next) = current {
            *next = Box::new(c_node);
        }

        // print all
        let mut current = &my_list;
        while let ListNode(v, next) = current {
            print_out.print(format!("List: {} ", v));
            current = next;
        }
        print_out.print_line(" End ");
    }
}

// control flow
fn my_flow(print_out: &mut MyPrint) {
    // loop
    print_out.print_line("=== loop ===");
    let mut i = 0;
    loop {
        i += 1;
        print_out.print(format!("loop: {}. ", i));
        if i > 8 {
            break;
        }
    }
    print_out.print("\n");
    // while
    print_out.print_line("=== while ===");
    i = 0;
    while i < 8 {
        i += 1;
        print_out.print(format!("while: {}. ", i));
    }
    print_out.print("\n");

    // for
    print_out.print_line("=== for ===");
    for i in 0..8 {
        print_out.print(format!("for: {}. ", i));
    }
    print_out.print("\n");

    // match
    print_out.print_line("=== match ===");
    let what_match = "Yes";
    match what_match {
        "Yes" => {
            print_out.print_line("Match: Yes");
        }
        _ => {
            print_out.print_line("Match: No");
        }
    }
    let match_case: [&str; 2] = ["Yes", "No"];
    match what_match {
        f if f == match_case[0] => {
            print_out.print_line("Match: if if f == Yes");
        }
        _ => {
            print_out.print_line("Match: if if f == No");
        }
    }
}
