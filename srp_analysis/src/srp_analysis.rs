use crate::common::*;
use crate::tasks::*;
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
fn load_factor(task: &Task) -> f32 {
    return wcet(&task) / task.inter_arrival as f32;
}

/// Returns worst case execution time of a task
fn wcet(task: &Task) -> f32 {
    return task.trace.end as f32 - task.trace.start as f32;
}

/// Returns the response time of a task
fn response_time(
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

/// Returns the blocking time of a task
fn blocking_time(
    task: &Task,
    tasks: &Vec<Task>,
    ip: &HashMap<String, u8>,
    tr: &HashMap<String, HashSet<String>>,
) -> f32 {
    let mut blocking_time: f32 = 0.0;
    let mut task_prio = 0;
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
            if let Some(r_prio) = ip.get(r) {
                if t.prio < task.prio && r_prio >= &task.prio {
                    let wcet_resource = wcet_resource(&t.trace, r);
                    if wcet_resource > blocking_time {
                        blocking_time = wcet_resource;
                    }
                }
            }
        }
    }

    return blocking_time;
}

/// Returns the worst case execution time of a trace
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

/// Returns either the approx preemption time or the exact preemption time of a task
fn preemption(
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
        let busy_period = wcet(task) + blocking_time(task, tasks, ip, tr);
        preemption = preemption_exact(task, tasks, ip, tr, busy_period, busy_period);
    }

    return preemption;
}

/// Returns approx preemption time
fn preemption_approx(task: &Task, tasks: &Vec<Task>, ip: &HashMap<String, u8>) -> f32 {
    let mut preemption = 0.0;

    for t in tasks {
        if t.prio > task.prio {
            preemption += wcet(task) * (task.deadline as f32 / t.inter_arrival as f32).ceil();
        }
    }

    return preemption;
}

/// Returns exact preemption time, based on the response time recurrence eq.
/// 7.22 in Hard Real-Time Computing Systems.
fn preemption_exact(
    task: &Task,
    tasks: &Vec<Task>,
    ip: &HashMap<String, u8>,
    tr: &HashMap<String, HashSet<String>>,
    busy_period: f32,
    mut prev: f32,
) -> f32 {
    let mut curr = 0.0;

    if busy_period > task.deadline as f32 {
        panic!("task non-schedulable: deadline miss!")
    } else {
        curr += busy_period;
        for t in tasks {
            if t.prio > task.prio {
                let preemption = (prev / t.inter_arrival as f32).ceil() * wcet(t);
                curr += preemption;
            }
        }
        if curr == prev {
            return curr - busy_period;
        } else {
            preemption_exact(task, tasks, ip, tr, busy_period, curr)
        }
    }
}

/// Returns a compiled analysis of the system
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

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_preemption() {
        let t1 = Task {
            id: "T1".to_string(),
            prio: 1,
            deadline: 100,
            inter_arrival: 100,
            trace: Trace {
                id: "T1".to_string(),
                start: 0,
                end: 10,
                inner: vec![],
            },
        };

        let t2 = Task {
            id: "T2".to_string(),
            prio: 2,
            deadline: 200,
            inter_arrival: 200,
            trace: Trace {
                id: "T2".to_string(),
                start: 0,
                end: 30,
                inner: vec![
                    Trace {
                        id: "R1".to_string(),
                        start: 10,
                        end: 20,
                        inner: vec![Trace {
                            id: "R2".to_string(),
                            start: 12,
                            end: 16,
                            inner: vec![],
                        }],
                    },
                    Trace {
                        id: "R1".to_string(),
                        start: 22,
                        end: 28,
                        inner: vec![],
                    },
                ],
            },
        };

        let t3 = Task {
            id: "T3".to_string(),
            prio: 3,
            deadline: 50,
            inter_arrival: 50,
            trace: Trace {
                id: "T3".to_string(),
                start: 0,
                end: 30,
                inner: vec![Trace {
                    id: "R2".to_string(),
                    start: 10,
                    end: 20,
                    inner: vec![],
                }],
            },
        };

        // builds a vector of tasks t1, t2, t3
        let tasks: Tasks = vec![t1, t2, t3];

        let (ip, tr) = pre_analysis(&tasks);
        let exact = srp_analysis(&tasks, &ip, &tr, false);
        assert_eq!(exact[0].4, 90.0);
        assert_eq!(exact[1].4, 60.0);
        assert_eq!(exact[2].4, 0.0);

        let approx = srp_analysis(&tasks, &ip, &tr, true);
        assert_eq!(approx[0].4, 30.0);
        assert_eq!(approx[1].4, 120.0);
        assert_eq!(approx[2].4, 0.0);
    }
}
