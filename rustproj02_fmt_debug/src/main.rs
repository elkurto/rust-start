

fn main() {
  // based on https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
  print_debug00_struct();

  print_debug01_mapstruct();
}

// :macro: #[derive(Debug)]  auto implements :method: fmt:Debug.
// :method: fmt:Debug is called in string templated via {:?} placeholder.
// (see below)
#[derive(Debug)]
struct TupleStruct123(i32, i32, i32);

#[derive(Debug)]
struct DeepTupleStruct123(TupleStruct123,TupleStruct123);

fn print_debug00_struct() {

  println!("\nin print_debug00_struct()\n");
  // instantiate TupleStruct123 
  let ta =TupleStruct123(51,66,79);
  let tb =TupleStruct123(123,456,789);
  // duplicate (using "struct-update-syntax") to create
  // an instance of DeepTupleStruct123 from ta and tb.
  // https://doc.rust-lang.org/book/ch05-01-defining-structs.html
  let dts =DeepTupleStruct123(TupleStruct123(ta.0, ta.1, ta.2), TupleStruct123(tb.0,tb.1,tb.2));

  // use {:?} to translated struct to string via  fmt::Debug 
  // 
  println!("  ta ={:?}", ta);    // invoke fmt::Debug(ta)
  println!("  tb ={:?}", tb);    // invoke fmt::Debug(tb)
  println!("  dts.0 ={:?} ::: dts.1 ={:?}", dts.0, dts.1);
  println!("  dts ={:?}", dts);  // invoke fmt::Debug(dts)

  // output:
  // in print_debug00_struct()
  // 
  //   ta =TupleStruct123(51, 66, 79)
  //   tb =TupleStruct123(123, 456, 789)
  //   dts.0 =TupleStruct123(51, 66, 79) ::: dts.1 =TupleStruct123(123, 456, 789)
  //   dts =DeepTupleStruct123(TupleStruct123(51, 66, 79), TupleStruct123(123, 456, 789))

}

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8
}

fn print_debug01_mapstruct() {
  let pa = Person { name: "pa", age: 23 };
  let pb = Person { name: "pb", age: 25 };
  
  println!("\nin print_debug01_mapstruct()");
  println!("  \nprint {{:?}} both pa an pb");
  println!("  pa = {:?}", pa);
  println!("  pb = {:?}", pb);

  println!("  \npretty print {{:#?}} both pa an pb");
  println!("  pa = {:#?}", pa);
  println!("  pb = {:#?}", pa);
  
  // output:
  //   in print_debug01_mapstruct()
  // 
  //   print {:?} both pa an pb
  //   pa = Person { name: "pa", age: 23 }
  //   pb = Person { name: "pb", age: 25 }
  // 
  //   pretty print {:#?} both pa an pb
  //   pa = Person {
  //     name: "pa",
  //     age: 23,
  //   }
  //   pb = Person {
  //     name: "pa",
  //     age: 23,
  //   }

}