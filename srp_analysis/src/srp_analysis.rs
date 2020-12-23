
use crate::common::*;

pub fn response_time(blocking_time: i32, wcet: i32, preemption: i32) -> i32 {
    return blocking_time + wcet + preemption;
}

pub fn blocking_time(task: Task) -> u32 {
    println!("task {:#?}", task);
    let mut blocking_time = task.trace.end - task.trace.start;
    
    let inner_traces = inner_trace(task.trace.inner);
    // println!("inner_trace = {:#?}", inner_traces);

    for trace in inner_traces {
        let trace_blocking = trace.end - trace.start;
        if trace_blocking > blocking_time {
            blocking_time = trace_blocking;
        }
    }

    println!("blocking_time = {:#?}", blocking_time);
    return blocking_time;
}

/// Retrives the inner traces of a task
pub fn inner_trace(trace: Vec<Trace>) -> Vec<Trace> {
    let mut traces: Vec<Trace> = [].to_vec();
    for t in trace {
        traces.push(t.clone());
        traces.extend(inner_trace(t.inner));
    }

    return traces
}