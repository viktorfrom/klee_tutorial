
pub fn response_time(blocking_time: i32, wcet: i32, preemption: i32) -> f32 {
    return blocking_time + wcet + preemption;
}