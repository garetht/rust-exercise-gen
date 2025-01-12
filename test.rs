fn print_string(s: String) {

}

fn main() {
    let s = String::from("hello");
    let y = &mut s;
    println!("{} {}", s, y);
}
