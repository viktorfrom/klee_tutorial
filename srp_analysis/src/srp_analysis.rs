use crate::common::*;

pub fn total_load_factor(tasks: Vec<Task>) -> f32 {
    let mut total_load_factor: f32 = 0.0;

    for task in tasks {
        total_load_factor += load_factor(&task);
    }

    return total_load_factor;
}

pub fn load_factor(task: &Task) -> f32 {
    return wcet(&task) / task.inter_arrival as f32
}

pub fn wcet(task: &Task) -> f32 {
    return task.trace.end as f32 - task.trace.start as f32;
}

pub fn response_time(task: &Task) -> f32 {
    return blocking_time(&task) + wcet(&task) + preemption(&task);
}

pub fn blocking_time(task: &Task) -> f32 {
    // println!("task {:#?}", task);
    let mut blocking_time = wcet(&task);

    let inner_traces = inner_trace(&task.trace.inner);
    // println!("inner_trace = {:#?}", inner_traces);

    for trace in inner_traces {
        let trace_wcet = trace.end as f32 - trace.start as f32;
        let mut trace_blocking = 0.0;

        if trace_wcet > trace_blocking {
            trace_blocking = trace_wcet;
        }

        blocking_time += trace_blocking;
    }

    // println!("blocking_time = {:#?}", blocking_time);
    return blocking_time;
}

/// Retrives the inner traces of a task
pub fn inner_trace(traces: &Vec<Trace>) -> Vec<Trace> {
    let mut inner_traces: Vec<Trace> = [].to_vec();
    for trace in traces {
        inner_traces.push(trace.clone());
        inner_traces.extend(inner_trace(&trace.inner));
    }

    return inner_traces;
}

pub fn preemption(task: &Task) -> f32 {
    return 0.0
}