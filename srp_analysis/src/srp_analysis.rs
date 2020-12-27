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
    approx: bool,
) -> f32 {
    return blocking_time(&task, tasks, ip, tr)
        + wcet(&task)
        + preemption(&task, tasks, ip, tr, approx);
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
pub fn preemption(
    task: &Task,
    tasks: &Vec<Task>,
    ip: &HashMap<String, u8>,
    tr: &HashMap<String, HashSet<String>>,
    approx: bool,
) -> f32 {
    let mut preemption = 0.0;

    if approx {
        preemption = preemption_approx(task, tasks, ip);
    } else {
        preemption = preemption_exact(task, tasks, ip, tr);
    }

    return preemption;
}

pub fn preemption_approx(task: &Task, tasks: &Vec<Task>, ip: &HashMap<String, u8>) -> f32 {
    let mut preemption = 0.0;

    for t in tasks {
        if t.prio > task.prio {
            preemption += wcet(task) * (task.deadline as f32 / t.inter_arrival as f32).ceil();
        }
    }

    return preemption;
}

pub fn preemption_exact(
    task: &Task,
    tasks: &Vec<Task>,
    ip: &HashMap<String, u8>,
    tr: &HashMap<String, HashSet<String>>,
) -> f32 {
    let mut preemption = 0.0;

    for t in tasks {
        if t.prio > task.prio {
            let busy_period = wcet(task) + blocking_time(task, tasks, ip, tr);
            preemption = response_time_rec(task, t, busy_period, 0.0);
        }
    }

    return preemption;
}

pub fn response_time_rec(task: &Task, t: &Task, busy_period: f32, mut curr: f32) -> f32 {
    if (curr - busy_period) > task.deadline as f32 {
        panic!("task non-schedulable: deadline miss!")
    } else {
        if curr == busy_period {
            return curr;
        } else {
            let curr = busy_period + (curr / t.inter_arrival as f32).ceil() * wcet(t);
            return response_time_rec(task, t, busy_period, curr);
        }
    }
}

pub fn srp_analysis(
    tasks: &Vec<Task>,
    ip: &HashMap<String, u8>,
    tr: &HashMap<String, HashSet<String>>,
    approx: bool,
) -> Vec<(String, f32, f32, f32, f32)> {
    let mut v = Vec::new();

    for t in tasks {
        v.push((
            t.id.to_string(),
            response_time(t, tasks, ip, tr, approx),
            wcet(t),
            blocking_time(t, tasks, ip, tr),
            preemption(t, tasks, ip, tr, approx),
        ))
    }

    return v;
}
