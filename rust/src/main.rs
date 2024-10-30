mod dummy_array;
use std::clone;

use dummy_array::DummyArrayVec;

fn main() {
    let mut my_dummy_array = DummyArrayVec::new(2).unwrap();
    print!("{}", my_dummy_array.repr());

    // // clone the array
    // let clone_dummy_array = my_dummy_array.clone();
    // assert_eq!(true, my_dummy_array.partial_eq(&clone_dummy_array), "The two arrays should be equal. ");

    // no value should exist in the array at this point
    assert_ne!(true, my_dummy_array.exists(0), "Value 0 should not exist. ");
    // the length+1 value should not be recognized as a stored value
    assert_ne!(true, my_dummy_array.exists(1), "Value 1 should not exist. ");

    // add a value to the empty array
    assert_eq!(true, my_dummy_array.add(1).unwrap(), "Value 1 should be added. ");
    // add a value to the array
    assert_eq!(true, my_dummy_array.add(0).unwrap(), "Value 0 should be added. ");

    print!("{}", my_dummy_array.repr());

    // add twice the same value
    assert_eq!("Value already exists. ", my_dummy_array.add(1).unwrap_err(), "Value 1 should not be added. ");
    // add a value that is not valid
    assert_eq!("Not a valid value. ", my_dummy_array.add(-1).unwrap_err(), "Value -1 should not be added. ");
    // add a value greater than the length
    assert_eq!(true, my_dummy_array.add(2).unwrap(), "Value 2 should be added. ");

    print!("{}", my_dummy_array.repr());

    // search for the value that was added
    assert_eq!(true, my_dummy_array.exists(1), "Value 1 should exist. ");

    // get the value that was added
    assert_eq!(1, my_dummy_array.get(1).unwrap(), "Value 1 should be retrieved. ");
    
    // remove a non-existing value
    assert_eq!("Value not found. ", my_dummy_array.remove(4).unwrap_err(), "Value 4 should not be removed. ");

    print!("{}", my_dummy_array.repr());

    // remove the value that was added
    assert_eq!(true, my_dummy_array.remove(1).unwrap(), "Value 1 should be removed. ");

    print!("{}", my_dummy_array.repr());

    // search for the value that was removed
    assert_eq!(false, my_dummy_array.exists(1), "Value 1 should not exist. ");
}
