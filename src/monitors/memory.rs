use sysinfo::System;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Memory_Stats{
    pub memory_type : String,
    pub memory_usage : f64,
}

pub struct Memory_Monitor;

impl Memory_Monitor{


    pub fn get_memory_usage_percentage(self : &Self) -> Memory_Stats{
        let total_ram = self.get_memory_total();
        let current_ram = self.get_memory_usage();
        Memory_Stats{
            memory_type : "RAM_USAGE_PC".to_string(),
            memory_usage : 100. * current_ram.memory_usage / total_ram.memory_usage,
        }
    }

    pub fn get_memory_usage(self: &Self) -> Memory_Stats{
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        Memory_Stats{
            memory_type : "RAM_USAGE".to_string(),
            memory_usage : sys.used_memory() as f64,
        }
    }


    pub fn get_memory_total(self: &Self) -> Memory_Stats{
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        Memory_Stats{
            memory_type : "RAM_TOTAL".to_string(),
            memory_usage : sys.total_memory() as f64,
        }
    }


    pub fn get_swap_usage(self: &Self) -> Memory_Stats{
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        Memory_Stats{
            memory_type : "SWAP_USAGE".to_string(),
            memory_usage : sys.used_swap() as f64,
        }
    }

    pub fn get_swap_total(self: &Self) -> Memory_Stats{
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        Memory_Stats{
            memory_type : "SWAP_TOTAL".to_string(),
            memory_usage : sys.total_swap() as f64,
        }
    }


    pub fn get_swap_usage_percentage(self : &Self) -> Memory_Stats{
        let total_swap = self.get_swap_total();
        let current_swap = self.get_swap_usage();
        Memory_Stats{
            memory_type : "SWAP_USAGE_PC".to_string(),
            memory_usage : 100. * current_swap.memory_usage / total_swap.memory_usage,
        }
    }
}