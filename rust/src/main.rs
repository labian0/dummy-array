mod dummy_array;
use dummy_array::DummyArray;

fn main() {
    let my_dummy_array = DummyArray::new(1);
    print!("indexing tab : {:?}", my_dummy_array.get_index_tab());
    print!("storing tab : {:?}", my_dummy_array.get_storing_tab());
}
