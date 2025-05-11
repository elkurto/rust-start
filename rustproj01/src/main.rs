fn main() {
    println!("This program prints various objects");
    print_format_01();
    print_format_02();
}

fn print_format_01() {
    let x =5 + 90 +5;
    println!("\nprint_format_01()");
    println!("  The value of x is: {}", x);

    // output :
    // print_format_01()
    //   The value of x is: 100
}

fn print_format_02() {
    println!("\nprint_format_02()");

    println!("  Print with positional args");
    println!("  {{0}}= {0}, {{1}} = {1}. {{2}} ={2}, this is {{0}}, {0}, again.", "Zero", "One", "Two");

    // output:
    // print_format_02()
    //   Print with positional args
    //   {0}= Zero, {1} = One. {2} =Two, this is {0}, Zero, again.
}

