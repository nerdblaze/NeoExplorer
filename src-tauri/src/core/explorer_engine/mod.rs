/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: explorer_engine
 * File Name: mod.rs
 * Author: B74Z3
 * Description: This module handles all explorer operations
 ******************************************************************************/

pub mod explorer_service;
pub mod file_service;

/******************************************************************************
 * Libraries:
 ******************************************************************************/
// Standard Libraries
use std::collections::HashMap;

// External Crates
// use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

// Internal Modules

/******************************************************************************
 * Constants:
 ******************************************************************************/

/******************************************************************************
 * Structures and Enums:
 ******************************************************************************/
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    categories: HashMap<String, HashMap<String, String>>,
}

/******************************************************************************
 * Implementations:
 ******************************************************************************/

/******************************************************************************
* Functions:
 ******************************************************************************/
