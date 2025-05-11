fn main() {
  println!("This program prints various objects");
  print_format_01();
  print_format_02_positional_args();
  print_format_03_named_args();
  print_format_04_format_numbers()
}

fn print_format_01() {
  let x =5 + 90 +5;
  println!("\nprint_format_01()");
  println!("  The value of x is: {}", x);

  // output :
  // print_format_01()
  //   The value of x is: 100
}

fn print_format_02_positional_args() {
  println!("\nprint_format_02_positional_args()");

  println!("  Print with positional args");
  println!("  {{0}}= {0}, {{1}} = {1}. {{2}} ={2}, this is {{0}}, {0}, again.", "Zero", "One", "Two");

  // output:
  // print_format_02_positional_args()
  //   Print with positional args
  //   {0}= Zero, {1} = One. {2} =Two, this is {0}, Zero, again.
}

fn print_format_03_named_args() {
  println!("\nprint_format_03_named_args()");

  println!("  Print with named args");
  println!("  {{noun}} {{verb}} {{direct_object}} ={noun} {verb} {direct_object}"
      , noun="Huckleberry Finn"
      , verb="painted"
      , direct_object = "the fence"
  )

  // output:
  //   print_format_03_named_args()
  //   Print with named args
  //   {noun} {verb} {direct_object} =Huckleberry Finn painted the fence

}

fn print_format_04_format_numbers() {
  
  println!("\nprint_format_04_format_numbers()");
  println!("  Base 10:               {}",   69420); // 69420
  println!("  Base 2 (binary):       {:b}", 69420); // 10000111100101100
  println!("  Base 8 (octal):        {:o}", 69420); // 207454
  println!("  Base 16 (hexadecimal): {:x}", 69420); // 10f2c


  println!("  {number:>5}", number=8);  // ____8  // right justify ">" , width=5, leftpad='space'
  println!("  {number:<5}", number=8);  // 8____  // left  justify "<" , width=5, rightpad='space'


  println!("  {number:0>5}", number=8); // 00008  // right justify, width=5, leftpad="0"
  println!("  {number:_<5}", number=8); // 8____  // left  justify, width=5, rightpad="_"
  println!("  {number:_>width$}", number=8, width=5); // ____8

  println!("  {number:0>5.2}", number=8.9); // 08.90  // format nn.mm, leftpad="0

  // pull format values from context variables note: use of $
  //  

  let my_num =7.3;
  let my_width =8;
  let my_precision =2;

  println!("  my_float ={my_num:0>my_width$.my_precision$}"); // 00007.30 
}

