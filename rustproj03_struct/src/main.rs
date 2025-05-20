#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rect {
    top: i64,
    left: i64,
    right: i64,
    bottom: i64,
}

fn main() {

    ex00_c_like_struct_person_and_rect();
    ex01_enum();
}

fn ex00_c_like_struct_person_and_rect() {
    // https://doc.rust-lang.org/rust-by-example/custom_types/structs.html

    println!("\nex00_c_like_struct_person_and_rect()");
    let jj =Person { name:String::from("jj"), age:18};
    println!("  jj = {:?}", jj);
    // output: jj = Person { name: "jj", age: 18 }


    let mut rect = Rect{top: 20, left: 20, right: 30, bottom: 30};
    println!("  rect = {:?}", rect);
    // output: rect = Rect { top: 20, left: 20, right: 30, bottom: 30 }

    rect.top =50;
    rect.left =50;
    rect.right =60;
    rect.bottom =60;

    println!("  rect = {:#?}", rect);
    // {:#? prints debug output:
    //   rect = Rect {
    //     top: 50,
    //     left: 50,
    //     right: 60,
    //     bottom: 60,
    //   }


    //output:
    // ex00_c_like_struct_person_and_rect()
    //   jj = Person { name: "jj", age: 18 }
    //   rect = Rect { top: 20, left: 20, right: 30, bottom: 30 }
    //   rect = Rect {
    //     top: 50,
    //     left: 50,
    //     right: 60,
    //     bottom: 60,
    //   }

}


enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// print a message
fn inspect_web_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("  page loaded"),
        WebEvent::PageUnload => println!("  page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("  pressed '{}'.", c),
        WebEvent::Paste(s) => println!("  pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("  clicked at x={}, y={}.", x, y);
        },
    }
}

fn ex01_enum() {
    println!("\nex01_enum()");
    // https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect_web_event(pressed);  // pressed "x"
    inspect_web_event(pasted);   // pasted "my text"
    inspect_web_event(click);    // clicked at x-20, y=20
    inspect_web_event(load);     // page loaded
    inspect_web_event(unload);   // page unloaded

    // output:
    //     ex01_enum()
    //     pressed 'x'.
    //     pasted "my text".
    //     clicked at x=20, y=80.
    //     page loaded
    //     page unloaded
}

