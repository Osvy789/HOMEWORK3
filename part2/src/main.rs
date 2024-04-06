use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

const NUM_SENSORS: usize = 8;
const HOURS_TO_SIMULATE: usize = 24;

fn main() {
    let readings = Arc::new(Mutex::new(Vec::new()));
    let start_time = SystemTime::now();
    let mut handles = vec![];
    for _ in 0..NUM_SENSORS {
        let readings_clone = Arc::clone(&readings);
        let start_time_clone = start_time;
        let handle = thread::spawn(move || {
            simulate_sensor(readings_clone, start_time_clone);
        });
        handles.push(handle);
    }
    for hour in 0..HOURS_TO_SIMULATE {
        thread::sleep(Duration::from_secs(1)); 
        let current_readings = {
            let mut readings = readings.lock().unwrap();
            let current_readings = readings.clone();
            readings.clear(); 
            current_readings
        };

        if !current_readings.is_empty() {
            let report = generate_report(&current_readings);
            println!("Hour {}: {:?}", hour + 1, report);
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }

}

fn simulate_sensor(readings: Arc<Mutex<Vec<(SystemTime, i32)>>>, start_time: SystemTime) {
    let mut rng = rand::thread_rng();
    while SystemTime::now().duration_since(start_time).unwrap().as_secs() < (HOURS_TO_SIMULATE * 3600) as u64 {
        let temp = rng.gen_range(-100..=70);
        let now = SystemTime::now();
        {
            let mut readings = readings.lock().unwrap();
            readings.push((now, temp));
        }
        thread::sleep(Duration::from_secs(1)); 
    }
}

fn generate_report(readings: &[(SystemTime, i32)]) -> (Vec<i32>, Vec<i32>, (SystemTime, SystemTime, i32)) {
    let mut sorted_readings = readings.to_vec();
    sorted_readings.sort_by_key(|k| k.1);
    let lowest_five: Vec<i32> = sorted_readings.iter().take(5).map(|x| x.1).collect();
    let highest_five: Vec<i32> = sorted_readings.iter().rev().take(5).map(|x| x.1).collect();
    let largest_diff_interval = find_largest_temp_diff_interval(readings);
    (highest_five, lowest_five, largest_diff_interval)
}


fn find_largest_temp_diff_interval(readings: &[(SystemTime, i32)]) -> (SystemTime, SystemTime, i32) {
    if readings.len() < 2 {
        return (SystemTime::now(), SystemTime::now(), 0);
    }
    let mut sorted_readings = readings.to_vec();
    sorted_readings.sort_by_key(|k| k.0);
    let mut largest_diff = 0;
    let mut start_time_of_largest_diff = SystemTime::now();
    let mut end_time_of_largest_diff = SystemTime::now();
    for i in 0..sorted_readings.len() - 1 {
        let current_diff = (sorted_readings[i + 1].1 - sorted_readings[i].1).abs();
        if current_diff > largest_diff {
            largest_diff = current_diff;
            start_time_of_largest_diff = sorted_readings[i].0;
            end_time_of_largest_diff = sorted_readings[i + 1].0;
        }
    }
    (start_time_of_largest_diff, end_time_of_largest_diff, largest_diff)
}



