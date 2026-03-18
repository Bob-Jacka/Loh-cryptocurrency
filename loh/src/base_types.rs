use std::sync::{Arc, Mutex};

/**
Very abstract class for objects in blockchain, contains basic methods
*/
pub trait Object {
    fn new() -> Self;
    fn destroy(&mut self);
}

/**
Interface for saving data, locally or globally
*/
pub trait ISavable {
    fn save();
}

/**
Interface for loading data, locally or globally
*/
pub trait ILoadable {
    fn load();
}
