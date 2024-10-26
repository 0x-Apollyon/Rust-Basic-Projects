fn main() {
    let mut main_vector = Vec::new();
    main_vector.push(1);
    main_vector.push(4);
    main_vector.push(3);

    let vector_two = vec![4 , 5 , 6];

    println!("The even values in main vector are {:?}" , filter_even(main_vector));
    println!("The even values in vector two are {:?}" , filter_even(vector_two));

}

fn filter_even(vector : Vec<i32>) -> Vec<i32> {
    let mut returnable_vector = Vec::new();
    for value in vector{
        if value % 2 == 0 {
            returnable_vector.push(value);
        }
    }
    return returnable_vector;
}