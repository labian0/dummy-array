use std::{any::Any, clone, ops::Index, vec::Vec};

/// Dummy array implementation using vectors. <br/>
/// <br/>
/// indexing_tab: Vec<Box<i64>> - vector of pointers to the stored values in the storing_tab. <br/>
/// storing_tab: Vec<i64> - vector of stored values. <br/>
/// counter: i64 - index of the first writtable slot in the dummy array storing tab.  <br/>
#[derive(Debug)]
pub struct DummyArrayVec {
    indexing_tab: Vec<&'static mut Box<i64>>,
    storing_tab: Vec<i64>,
    counter: i64,
}

impl DummyArrayVec {

    /// Creates a new dummy array with a given capacity. <br/>
    pub fn new(capacity: i64) -> Result<Self, &'static str> 
    {
        if !DummyArrayVec::is_value_valid(capacity)
        {
            return Err("Not a valid capacity. ");
        }
        
        let mut new= Self {
                indexing_tab: Vec::with_capacity(capacity as usize),
                storing_tab: Vec::with_capacity(capacity as usize),
                counter: 0,
        };
        new.grow(capacity+1);

        Ok(new)
    }

    /// Searches for a value in the dummy array. <br/>
    /// Returns true if the value is found, false otherwise. <br/>
    pub fn exists(&mut self, value: i64) -> bool 
    {
        if !DummyArrayVec::is_value_valid(value) || value >= self.indexing_tab.len() as i64
        {
            return false;
        }
        let search_result = self.indexing_tab.get(value as usize);
        return if search_result.is_none() {false} else {***search_result.unwrap() == value};
    }

    /// Try to add a value to the dummy array. <br/>
    /// Returns true if the value was added, false otherwise. <br/>
    pub fn add(&mut self, value: i64) -> Result<bool, &str> 
    {
        if self.exists(value)
        {
            return Err("Value already exists. ");
        }
        else if !DummyArrayVec::is_value_valid(value)
        {
            return Err("Not a valid value. ");
        }
        if value >= self.indexing_tab.len() as i64
        {
            self.grow(value);
        } 
        else
        {
            *self.indexing_tab[value as usize] = Box::new(self.storing_tab[self.counter as usize]);
            **self.indexing_tab[value as usize] = value;
        }
        self.refresh_counter();
        Ok(true)
    }

    /// Try to remove a value from the dummy array. <br/>
    /// Returns true if the value was removed, false otherwise. <br/>
    pub fn remove(&mut self, value: i64) -> Result<bool, &str> 
    {
        if !self.exists(value) 
        {
            return Err("Value not found. ");
        }
        **self.indexing_tab[value as usize] = self.indexing_tab.len() as i64 + 1;
        self.refresh_counter();

        Ok(true)
    }

    /// Try to get a value from the dummy array. <br/>
    /// Returns the value if it was found, an error message otherwise. <br/>
    pub fn get(&mut self, value: i64) -> Result<i64, &str> 
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
            Ok(**self.indexing_tab[value as usize])
        }
    }

    // /// Returns a copy of the dummy array. <br/>
    // pub fn clone(&self) -> Self 
    // {
    //     Self {
    //         indexing_tab: .clone(),
    //         storing_tab: self.storing_tab.clone(),
    //         counter: self.counter,
    //     }
    // }

    /// Resizes the dummy array to a new capacity, based on a value greatter than 
    /// the dummy array current lenght. <br/>
    fn grow(&mut self, value: i64) -> ()
    {
        let current_capacity: usize = self.indexing_tab.len();
        let new_capacity: usize = self.indexing_tab.len() + value as usize - 1;

        for p in &mut self.indexing_tab
        {
            if ***p == (current_capacity as i64 + 1)
            {
                ***p = new_capacity as i64 + 1;
            }
        }

        let mut index: usize = current_capacity;
        self.indexing_tab.resize_with(new_capacity, 
            || {
                self.storing_tab.push(new_capacity as i64 + 1);
                let tmp = Box::new(self.storing_tab[index]);
                index += 1;
                Box::leak(tmp)
            }
        );
    }

    /// Refreshes the counter value to the first writtable slot in the dummy array storing tab. <br/>
    /// <ins>NB</ins>: a writtable slot is a slot with a value greatter than the dummy array lenght. <br/>
    fn refresh_counter(&mut self) -> ()
    {
        let mut index: usize = 0;

        while index < self.indexing_tab.len() 
                && **self.indexing_tab[index as usize] < self.indexing_tab.len() as i64 + 1
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
        value >= 0 && value < i64::MAX 
    }

    pub fn repr(&self) -> String
    {
        let mut repr = String::from("------\nDummyArrayVec repr :\n");
        repr.push_str(&format!("indexing_tab: {:?}\n", self.indexing_tab));
        repr.push_str(&format!("storing_tab: {:?}\n", self.storing_tab));
        repr.push_str(&format!("counter: {}\n------\n", self.counter));
        repr
    }

    pub fn partial_eq(&self, other: &Self) -> bool
    {
        self.indexing_tab == other.indexing_tab
        && self.storing_tab == other.storing_tab
        && self.counter == other.counter
    }
}