#![allow(dead_code)]
use std::{ptr, vec::Vec, clone::Clone};

/// Dummy array implementation using vectors. <br/>
/// <br/>
/// indexing_tab: Vec<Box<i64>> - vector of pointers to the stored values in the storing_tab. <br/>
/// storing_tab: Vec<i64> - vector of stored values. <br/>
/// counter: i64 - index of the first writtable slot in the dummy array storing tab.  <br/>
#[derive(Debug)]
pub struct DummyArrayVec {
    indexing_tab: Vec<*mut i64>,
    storing_tab: Vec<i64>,
    counter: i64,
}

/// Dummy array common behaviors. <br/>
pub trait DummyArray {
    fn exists(&mut self, value: i64) -> bool;
    fn add(&mut self, value: i64) -> Result<bool, &str>;
    fn remove(&mut self, value: i64) -> Result<bool, &str>;
    fn get(&mut self, value: i64) -> Result<i64, &str>;
    fn repr(&self) -> String;
    fn is_full(&self) -> bool;
    fn partial_eq(&self, other: &Self) -> bool;
}

impl DummyArray for DummyArrayVec {
    /// Searches for a value in the dummy array. <br/>
    /// Returns true if the value is found, false otherwise. <br/>
    fn exists(&mut self, value: i64) -> bool 
    {
        let search_result = self.indexing_tab.get(value as usize);
        return if search_result.is_none() {false} else { unsafe{ **search_result.unwrap() == value } };
    }

    /// Try to add a value to the dummy array. <br/>
    /// Returns true if the value was added, false otherwise. <br/>
    fn add(&mut self, value: i64) -> Result<bool, &str> 
    {
        if self.exists(value)
        {
            return Err("Value already exists. ");
        }
        else if !DummyArrayVec::is_value_valid(value)
        {
            return Err("Not a valid value. ");
        }
        else if value >= self.indexing_tab.len() as i64
        {
            self.grow((value+1) as usize);
        }
        else if self.is_full()
        {
            return Err("The dummy array is full. ");
        }
        self.refresh_counter();
        self.indexing_tab[value as usize] = ptr::from_ref(&self.storing_tab[self.counter as usize]).cast_mut();
        unsafe 
        {
            *self.indexing_tab[value as usize] = value;
        }

        Ok(true)
    }

    /// Try to remove a value from the dummy array. <br/>
    /// Returns true if the value was removed, false otherwise. <br/>
    fn remove(&mut self, value: i64) -> Result<bool, &str> 
    {
        if !self.exists(value) 
        {
            return Err("Value not found. ");
        }

        unsafe { *self.indexing_tab[value as usize] = self.indexing_tab.len() as i64; }

        Ok(true)
    }

    /// Try to get a value from the dummy array. <br/>
    /// Returns the value if it was found, an error message otherwise. <br/>
    fn get(&mut self, value: i64) -> Result<i64, &str> 
    {
        if !self.exists(value)
        {
            return Err("Value not found. ");
        }
        else if !DummyArrayVec::is_value_valid(value)
        {
            return Err("Not a valid value. ");
        }
        else
        {
            Ok(unsafe { *self.indexing_tab[value as usize] })
        }
    }

    /// Returns a string representation of the dummy array. <br/>
    fn repr(&self) -> String
    {
        let mut repr = String::from("------\nDummyArrayVec repr :\n");
        repr.push_str(&format!("indexing_tab: {:?}\n", self.indexing_tab));
        repr.push_str(&format!("storing_tab: {:?}\n", self.storing_tab));
        repr.push_str(&format!("counter: {}\n------\n", self.counter));
        repr
    }

    /// Returns true if the dummy array is full, false otherwise. <br/>
    /// <ins>NB</ins>: the dummy array is full if the counter is equal to the dummy array lenght
    /// (no writtable slot). <br/>
    fn is_full(&self) -> bool
    {
        self.counter == self.indexing_tab.len() as i64
    }

    /// Returns true if the dummy array is empty, false otherwise. <br/>
    fn partial_eq(&self, other: &Self) -> bool
    {
        self.indexing_tab == other.indexing_tab
        && self.storing_tab == other.storing_tab
        && self.counter == other.counter
    }
}

impl Clone for DummyArrayVec {
    /// Returns a copy of the dummy array. <br/>
    fn clone(&self) -> Self 
    {
        Self {
            indexing_tab: self.indexing_tab.clone(),
            storing_tab: self.storing_tab.clone(),
            counter: self.counter,
        }
    }
}

impl DummyArrayVec {

    /// Creates a new dummy array with a given capacity. <br/>
    pub fn new(capacity: usize) -> Result<Self, &'static str> 
    {
        if !DummyArrayVec::is_value_valid(capacity as i64)
        {
            return Err("Not a valid capacity. ");
        }
        
        let indexing_tab: Vec<*mut i64> = Vec::with_capacity(capacity);
        let storing_tab: Vec<i64> = Vec::with_capacity(capacity);

        let mut new= Self {
                indexing_tab,
                storing_tab,
                counter: 0,
        };
        new.grow(capacity);
        
        Ok(new)
    }

    /// Resizes the dummy array to a new capacity, based on a value greatter than 
    /// the dummy array current lenght, an reorient the indexing tab pointers 
    /// on the new address of the storing vec. <br/>
    fn grow(&mut self, new_capacity: usize) -> ()
    {
        let current_capacity: usize = self.indexing_tab.len();
        let mut values_to_repoint: Vec<i64> = Vec::new();

        if !(new_capacity < current_capacity)
        {
            if current_capacity != 0
            {
                values_to_repoint = self.storing_tab.clone();
            }

            let mut index: usize = current_capacity;
            self.indexing_tab.resize_with(new_capacity, 
                || 
                {
                    self.storing_tab.push(new_capacity as i64);
                    let new_access_ptr: *mut i64 = ptr::from_ref(&self.storing_tab[index]).cast_mut();
                    index += 1;
                    new_access_ptr
                }
            );

            if current_capacity != 0
            {
                self.back_on_track(current_capacity, new_capacity, values_to_repoint);
            }
        }
    }

    /// Reorients the indexing tab pointers on the new address of the storing vec afeter resizing
    /// and updates the writtable slots to the new capacity value. <br/>
    fn back_on_track(&mut self, old_capacity: usize, new_capacity: usize, values_to_repoint: Vec<i64>) -> ()
    {
        for index in 0..old_capacity
        {
            unsafe 
            {
                // get the value originally pointed by the pointer in the indexing tab
                let value_to_point: i64 = values_to_repoint[index];
                // get the new location of the value in the storing tab, relocated after resizing
                let new_location: *mut i64 = ptr::from_ref(&self.storing_tab
                    [{
                        // storing tab is a copy of its vec before resizing, 
                        // so just search for the new location of the same value
                        let mut index: usize = 0;
                        while index < new_capacity
                            && self.storing_tab[index] != value_to_point
                        {
                            index += 1;
                        }
                        index
                    }]).cast_mut();

                self.indexing_tab[index] = new_location;
                    
                // in the storing tab, the writtable slots defined before resizing are still marked 
                // with the old capacity value, so update them to the new capacity value
                if *self.indexing_tab[index] == (old_capacity as i64)
                {
                    *self.indexing_tab[index] = new_capacity as i64;
                }
            }
        }
    }

    /// Refreshes the counter value to the first writtable slot in the dummy array storing tab. <br/>
    /// <ins>NB</ins>: a writtable slot is a slot with a value greatter than the dummy array lenght. <br/>
    fn refresh_counter(&mut self) -> ()
    {
        let mut index: usize = 0;
        let writtable_value: i64 = self.indexing_tab.len() as i64;

        while index < self.indexing_tab.len()
                && self.storing_tab[index] < writtable_value
        {
            index += 1;
        }
        self.counter = index as i64;
    }

    /// A fonction to check if a value is valid. <br/>
    /// <ins>NB</ins>: value is valid if it is ge than 0 and less than i64::MAX, wich is the maximum storable value. <br/>
    /// Returns true if the value is valid, false otherwise. <br/>
    fn is_value_valid(value: i64) -> bool 
    {
        value >= 0 && value <= i64::MAX 
    }
}
