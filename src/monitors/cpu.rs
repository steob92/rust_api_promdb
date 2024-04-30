use sysinfo::System;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct CPU_Stats{
    pub cpu_id : usize,
    pub cpu_usage : f32,
}

pub struct CPU_Monitor;

impl CPU_Monitor{


    pub fn get_ncpu(self : &Self) -> usize {
        let mut sys: System = System::new();

        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu(); // Refreshing CPU information.

        sys.cpus().len()
    }

    pub fn get_cpu_usage( self : &Self, cpu_id : usize ) -> Result<f32, &'static str>{
        
        let cpus = self.get_cpu_usage_all()?;

        match cpu_id < cpus.len() {
            true => {                
                Ok(cpus[cpu_id])
            },
            false => {
                Err("CPU ID not found")
            }

        }

    }

    pub fn get_cpu_usage_all(self : &Self) -> Result<Vec<f32>, &'static str>{
                
        let mut sys: System = System::new();

        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu(); // Refreshing CPU information.

        let ncpu = sys.cpus().len();
        let mut cpu_vec = vec![0_f32; ncpu]; 
        for (i, cpu) in sys.cpus().iter().enumerate(){
            cpu_vec[i] =  cpu.cpu_usage();
        }
        Ok(cpu_vec)
        
    }

    pub fn print_cpu_usage(self: &Self) -> (){
                
        let cpus = self.get_cpu_usage_all().unwrap();
        println!("NB CPUs: {}", cpus.len());
        cpus.iter().enumerate().for_each( |(i, cpu)| {
            println!("CPU_{}: {:0.2}% ", i, cpu);
        });
        
    }

}
