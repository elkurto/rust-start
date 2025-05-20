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
}

fn ex00_c_like_struct_person_and_rect() {
    println!("\nex00_c_like_struct_person_and_rect()");
    let jj =Person { name:String::from("jj"), age:18};
    println!("  jj = {:?}", jj); // name


    let mut rect = Rect{top: 20, left: 20, right: 30, bottom: 30};
    println!("  rect = {:?}", rect);

    rect.top =50;
    rect.left =50;
    rect.right =60;
    rect.bottom =60;

    println!("  rect = {:#?}", rect);

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