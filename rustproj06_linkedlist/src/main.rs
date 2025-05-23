use rustproj06_linkedlist::*;


fn main() {
    let mut n1 =NodeInt32::new(1);

    n1.next = Some(Box::new(NodeInt32::new(2)));

    println!("n1={} ", n1);  // output: n1=1,2,None

    println!("n1.next.as_ref().unwrap ={}", n1.next.as_ref().unwrap());
    // output:
    //   n1=1,2,None
    //   n1.next.as_ref().unwrap =2,None
    // 


    // let mut list =ListInt32::new();
    //
    // list.push_i32(1);
    // list.push_i32(2);
    // list.push_i32(3);

}
