#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Rect {
    top: i64,
    left: i64,
    right: i64,
    bottom: i64,
}



fn main() {

    let jj =Person { name:String::from("jj"), age:18};
    println!("jj = {:?}", jj);


    let mut rect = Rect{top: 20, left: 20, right: 30, bottom: 30};
    println!("rect = {:?}", rect);

    rect.top =50;
    rect.left =50;
    rect.right =60;
    rect.bottom =60;

    println!("rect = {:?}", rect);
}
