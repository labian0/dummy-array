use std::{ptr, vec::Vec};

pub struct DummyArray {
    indexing_tab: Vec<*mut i64>,
    storing_tab: Vec<i64>,
    counter: i64,
}

impl DummyArray {

    //constructor
    pub fn new(capacity: i64) -> DummyArray {
        //storing vector is filled with capacity+1, meaning that we can write onto that index.
        let storing_tab: Vec<i64> = Vec::new();
        //indexing vector is filled with undefinded i64 pointers.
        let mut indexing_tab: Vec<*mut i64> = Vec::new();
        
        //indexing vector pointers are respectivelly set to one unique index address of the storing vector.
        for index in 0..capacity+1 {
            indexing_tab.push(ptr::from_ref(&storing_tab[index as usize]).cast_mut());
        }

        //the new DummyArray instance is returned.
        DummyArray {
            indexing_tab: indexing_tab,
            storing_tab: storing_tab,
            counter: 0,
        }
    }

    //index_tab getter
    pub fn get_index_tab(&self) -> Vec<*mut i64> {
        self.indexing_tab.clone()
    }

    //storing_tab getter
    pub fn get_storing_tab(&self) -> Vec<i64> {
        self.storing_tab.clone()
    }

}