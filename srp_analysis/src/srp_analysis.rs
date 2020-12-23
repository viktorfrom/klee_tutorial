use crate::common::*;

pub fn total_load_factor(tasks: Vec<Task>) -> f32 {
    let mut total_load_factor: f32 = 0.0;

    for task in tasks {
        let load_factor: f32 = task_wcet(&task) / task.inter_arrival as f32;
        total_load_factor += load_factor;
    }

    return total_load_factor;
}

pub fn task_wcet(task: &Task) -> f32 {
    return task.trace.end as f32 - task.trace.start as f32;
}


pub fn response_time(blocking_time: i32, wcet: i32, preemption: i32) -> i32 {
    return blocking_time + wcet + preemption;
}

pub fn blocking_time(task: Task) -> f32 {
    // println!("task {:#?}", task);
    let mut blocking_time = task_wcet(&task);

    let inner_traces = inner_trace(task.trace.inner);
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
pub fn inner_trace(traces: Vec<Trace>) -> Vec<Trace> {
    let mut inner_traces: Vec<Trace> = [].to_vec();
    for trace in traces {
        inner_traces.push(trace.clone());
        inner_traces.extend(inner_trace(trace.inner));
    }

    return inner_traces;
}
