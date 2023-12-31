struct Input {
    address: String,
    fizz: i8,
    flavor: i8,
    sweet: i8,
    notes: String,
}
fn main() {
    // After filling out the form, run ./pu.sh [State Abbreveation] [City Name] [Street Name] [Street Number]. The file will be created.
    let form = Input {
        address: "1743 King St, Alexandria, VA".to_string(),
        fizz: 10,
        flavor: 8,
        sweet: 9,
        notes: "Very good! Tastes a little watery.".to_string(),
    };
    let _ = data(form.address, form.fizz, form.flavor, form.sweet, form.notes);
}
fn data(loc: String, fizz: i8, flavor: i8, sweet: i8, notes: String) {
    let total = fizz + flavor + sweet;
    let total = total / 3;
    println!("Location: {loc}\nScore: {total}\nPoint Categories\n\tFizz: {fizz}\n\tFlavor: {flavor}\n\tSweetness: {sweet}\nNotes from taster:\n{notes}");
}
