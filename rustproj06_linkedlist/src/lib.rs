use std::fmt;

pub struct NodeInt32 {
  pub next : Option<Box<NodeInt32>>,
  pub val : i32,
}

impl NodeInt32 {
  // example:
  //   let n1 =NodeInt32::new( 5 );
  pub fn new(val:i32) -> NodeInt32 {
    return NodeInt32 {val, next: None};
  }


}

impl fmt::Display for NodeInt32 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // I needed to use Option::unwrap (@see https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap)
    // and to use Option::as_ref (@see https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref)

    if self.next.is_some() {
      write!(f, "{},{}", self.val, self.next.as_ref().unwrap())
    }else {
      write!(f, "{},None", self.val)
    }
  }
}

/*
pub struct ListInt32 {
  head : Option<Box<NodeInt32>>,
}

impl ListInt32 {
  pub fn new(val : Option<i32>) -> ListInt32 {
    let mut list = ListInt32 { head : None };

    if val.is_some() {
      list.head.unwrap().val =val;
    }

    return list;
  }

  pub fn push(&self, val : Option<i32>) {
    if val.is_none() {
      if self.head.is_none() {
        self.head =Some(Box::new(NodeInt32::new(val)));
      }else {
        // find tail node
        let curr =self.head;
        while !curr.unwrap().next.is_none() {
          curr =curr.unwrap().next;
        }
        curr.unwrap().next = Some(Box::new(NodeInt32::new(val)));
      }

    }
  }

  pub fn push_i32(&self, val : i32) {
    self.push(Some(val));
  }
}


 */

// impl fmt::Display for ListInt32 {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     f.write_str()
//   }
// }