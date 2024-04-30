#[macro_use] extern crate rocket;

mod monitors;
use std::{thread, time};
use monitors::{CPU_Monitor, CPU_Stats, Memory_Monitor};

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Value, json, Json};
// use rocket::response::status::{Custom};
use rocket::{Build, Rocket};

use once_cell::sync::Lazy;
use rocket_prometheus::prometheus::labels;
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec, Gauge, GaugeVec},
    PrometheusMetrics,
};

static CPU_USAGE: Lazy<Gauge> = Lazy::new(|| {
    Gauge::new("single_cpu", "CPU Usage")
        .expect("Could not create single_cpu")
});

static ALL_CPU_USAGE: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(opts!{"CPU_gauge", "CPU Usage"}, &["cpu_id"])
        .expect("Could not create CPU_gauge")
});


static MEMORY_TOTAL: Lazy<Gauge> = Lazy::new(|| {
    Gauge::new("memory_total", "Total Available Memory")
        .expect("Could not create memory_total")
});


static MEMORY_CURRENT: Lazy<Gauge> = Lazy::new(|| {
    Gauge::new("memory_current", "Current Available Memory")
        .expect("Could not create memory_current")
});


static SWAP_TOTAL: Lazy<Gauge> = Lazy::new(|| {
    Gauge::new("swap_total", "Total Available Swap Memory")
        .expect("Could not create swap_total")
});


static SWAP_CURRENT: Lazy<Gauge> = Lazy::new(|| {
    Gauge::new("swap_current", "Current Available Swap Memory")
        .expect("Could not create swap_current")
});







#[get("/cpu_usage/<id>")]
async fn get_usage(id:usize) ->  Json<CPU_Stats>{
    let mut monitor = CPU_Monitor;
    
    Json(
        CPU_Stats{
            cpu_id : id,
            cpu_usage : monitor.get_cpu_usage(id).unwrap(),
        }
    )
}


#[get("/cpu_usage")]
async fn get_usage_all() ->  Json<Vec<CPU_Stats>>{
    let mut monitor = CPU_Monitor;
    
    let cpu_stats = monitor.get_cpu_usage_all().unwrap();
    Json(
        cpu_stats.iter().enumerate().map( |(i, &usage)| {
            CPU_Stats{
                cpu_id : i,
                cpu_usage : usage,
            }
        }).collect::<Vec<CPU_Stats>>()
    )
}


fn monitor_cpu(){
    let mut monitor = CPU_Monitor;

    let cpu_stats = monitor.get_cpu_usage_all().unwrap();
    CPU_USAGE.set(cpu_stats[0] as f64); 
    (0..cpu_stats.len()).for_each( |i| {
        ALL_CPU_USAGE.get_metric_with_label_values(&[i.to_string().as_str()]).unwrap().set(cpu_stats[i] as f64); 

    })
}


fn monitor_memory(){
    let mut monitor = Memory_Monitor;
    let mem_current = monitor.get_memory_usage();
    let swap_current = monitor.get_swap_usage();

    MEMORY_CURRENT.set(mem_current.memory_usage); 
    SWAP_CURRENT.set(swap_current.memory_usage); 
}

// fn monitor_cpus(){
//     let mut monitor = CPU_Monitor;

//     let cpu_stats = monitor.get_cpu_usage_all().unwrap();
//     CPU_USAGE.set(cpu_stats[0] as f64); 
// }


fn initialization() -> (){
    let mut monitor = Memory_Monitor;
    let mem_total = monitor.get_memory_total();
    let swap_total = monitor.get_swap_total();

    MEMORY_TOTAL.set(mem_total.memory_usage); 
    SWAP_TOTAL.set(swap_total.memory_usage); 
}

#[catch(404)]
fn not_found() -> Value{
    json!("Not found!")
}

#[rocket::main]
async fn main() {


    initialization();

    let thread_join_handle = thread::spawn(|| {
        loop {
            // let ten_millis = time::Duration::from_millis(10);
            thread::sleep(time::Duration::from_secs(5));
            monitor_cpu();
            monitor_memory();
        }

    });

    let prometheus = PrometheusMetrics::new();

    prometheus
        .registry()
        .register(Box::new(CPU_USAGE.clone()))
        .unwrap();

    prometheus
        .registry()
        .register(Box::new(ALL_CPU_USAGE.clone()))
        .unwrap();

    prometheus
        .registry()
        .register(Box::new(MEMORY_CURRENT.clone()))
        .unwrap();

    prometheus
        .registry()
        .register(Box::new(MEMORY_TOTAL.clone()))
        .unwrap();


    prometheus
        .registry()
        .register(Box::new(SWAP_CURRENT.clone()))
        .unwrap();

    prometheus
        .registry()
        .register(Box::new(SWAP_TOTAL.clone()))
        .unwrap();


    let _ = rocket::build()
        .mount("/", routes![
            get_usage, 
            get_usage_all,
        ])
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .register("/", catchers![
            not_found,
            ])
        .launch()
        .await;

}
