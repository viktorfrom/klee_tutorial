
use crate::common::*;

pub fn response_time(blocking_time: i32, wcet: i32, preemption: i32) -> i32 {
    return blocking_time + wcet + preemption;
}

pub fn blocking_time(task: Task) {
    println!("task {:#?}", task);
}