struct Input {
    address: String,
    fizz: i8,
    flavor: i8,
    sweet: i8,
}
fn main() {
    let form = Input {
        address: "".to_string(),
        fizz: 0,
        flavor: 0,
        sweet: 0,
    };
    let _ = data(form.address, form.fizz, form.flavor, form.sweet);
}
fn data(loc: String, fizz: i8, flavor: i8, sweet: i8) {
    let total = fizz + flavor + sweet;
    let total = total / 3;
    println!("Location: {loc}\nScore: {total}\nPoint Categories\n\tFizz: {fizz}\n\tFlavor: {flavor}\n\tSweetness: {sweet}\n\nTaste-tested by: Luke T.")
}
