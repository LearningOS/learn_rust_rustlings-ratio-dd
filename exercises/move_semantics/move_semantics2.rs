// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)



    

    //1. Make another, separate version of the data that's in `vec0` and 
    //pass thatto `fill_vec` instead.
// fn main() {
 
//     let vec0 = Vec::new();
//     let mut vec1 = fill_vec(vec0.clone());

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }


    //2. Make `fill_vec` borrow its argument instead of taking ownership of it,
    //and then copy the data within the function in order to return an 
    //owned`Vec<i32>`
// fn main() {
//     let vec0 = Vec::new();
//     let mut vec1 = fill_vec(&vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec_owned = vec.clone();

//     vec_owned.push(22);
//     vec_owned.push(44);
//     vec_owned.push(66);

//     vec_owned
// }


    //3. Make `fill_vec` *mutably* borrow its argument (which will need to be
    //mutable), modify it directly, then not return anything. Then you can get 
    //rid of `vec1` entirely -- note that this will change what gets printed 
    //by the first `println!`
    //QUESTION: The proformance is not same compared to the other two methods.
    //Answer by Panda This is the right way hint3 want it to be.
    fn main() {
    let mut vec0 = Vec::new();

    

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {

    vec.push(22);
    vec.push(44);
    vec.push(66);
}
