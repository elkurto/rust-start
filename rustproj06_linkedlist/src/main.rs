use rustproj06_linkedlist::*;

fn main() {
    // let mut list =ListInt32::new();
    //
    // list.push_i32(1);
    // list.push_i32(2);
    // list.push_i32(3);

    let mut n1 =NodeInt32::new(1);
    let n2 =NodeInt32::new(2);
    n1.next = Some(Box::new(n2));


    println!("n1={} ", n1);
}
