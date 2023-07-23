//Tuples can be used as function arguemnts and return values

fn reverse(pair: (i32, bool)) -> (bool,i32) {
    //'let' can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param) // no ';' here means return this tuple
}

fn main() {
    let my_tuple : (i32, bool) = (5, true);
    println!("my_tuple is {:?}", my_tuple);
    println!("my_tuple reversed is {:?}", reverse(my_tuple));
}