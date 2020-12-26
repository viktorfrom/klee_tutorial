use crate::common::*;
use std::collections::{HashMap, HashSet};

/// Returns the total load factor of the CPU
pub fn tot_util(tasks: &Vec<Task>) -> f32 {
    let mut total_load_factor: f32 = 0.0;

    for t in tasks {
        total_load_factor += load_factor(&t);
    }

    return total_load_factor;
}

/// Returns the load factor of a single task
pub fn load_factor(task: &Task) -> f32 {
    return wcet(&task) / task.inter_arrival as f32;
}

pub fn wcet(task: &Task) -> f32 {
    return task.trace.end as f32 - task.trace.start as f32;
}

pub fn response_time(
    task: &Task,
    tasks: &Vec<Task>,
    ip: &HashMap<String, u8>,
    tr: &HashMap<String, HashSet<String>>,
) -> f32 {
    return blocking_time(&task, tasks, ip, tr) + wcet(&task) + preemption(&task);
}

pub fn blocking_time(
    task: &Task,
    tasks: &Vec<Task>,
    ip: &HashMap<String, u8>,
    tr: &HashMap<String, HashSet<String>>,
) -> f32 {
    let mut blocking_time: f32 = 0.0;
    let mut resources = &HashSet::new();

    // Retrieve resources used by the task
    match tr.get(&task.id) {
        Some(r) => resources = r,
        None => (),
    }

    // if the prio of t is lower than the task prio and t holds a resource with a 
    // resource prio >= task prio. then get max critical section of the resource. 
    for r in resources {
        for t in tasks {
            if (t.prio < task.prio) && ip.get(r).unwrap() >= &task.prio {
                let wcet_resource = wcet_resource(&task.trace, r);
                if wcet_resource > blocking_time {
                    blocking_time = wcet_resource;
                }
            }
        }
    }

    return blocking_time;
}

fn wcet_resource(trace: &Trace, resource: &str) -> f32 {
    let mut wcet: f32 = 0.0;

    if trace.id == resource {
        wcet = trace.end as f32 - trace.start as f32;
    } else if trace.inner.len() != 0 {
        for i in &trace.inner {
            let temp_wcet = wcet_resource(&i, resource);
            if temp_wcet > wcet {
                wcet = temp_wcet;
            }
        }
    }

    return wcet;
}

pub fn preemption(task: &Task) -> f32 {
    return 0.0;
}
