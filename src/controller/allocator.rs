//! This module contains the allocator implementation.
//! List of implemented allocators:
//! - SabaAllocator
//! - MaxMinAllocator
//!

use log::debug;
use saba::model::Model;
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::{collections::HashMap, fmt::Debug};

/// Allocator is a trait that defines the interface for the allocator.
pub trait Allocator: Debug {
    fn allocate(&mut self);
}

/// SabaAllocator is an allocator that uses the Saba scheme.
/// Saba is a bandwidth allocation scheme that uses a sensitivity model to
/// predict the slowdown of an application when the bandwidth is reduced.
/// The algorithm is described in the paper "Saba: Rethinking Datacenter Network
/// Allocation from Application’s Perspective" by M.R.S. Katebzadeh et al.
/// The algorithm is implemented in the `allocate` method.
/// The allocator uses the following tables:
/// - profile_table: a table that contains the profile of each application.
/// - slowdown_table: a table that contains the slowdown of each application
///  when the bandwidth is reduced.
///  The slowdown is calculated for each bandwidth value using the following formula:
///  slowdown = completion_time / baseline_completion_time
///  where completion_time is the completion time of the application when the
///  bandwidth is reduced, and baseline_completion_time is the completion time
///  of the application with unthrottled bandwidth.
///
#[derive(Debug)]
pub struct SabaAllocator<Sensitivity: Model> {
    sensitivity_table: HashMap<String, Box<dyn Model<Other = Sensitivity>>>,
    allocation_queue: AllocationQueue,
}

/// Trait implementation for SabaAllocator.
impl<Sensitivity: Model> Allocator for SabaAllocator<Sensitivity> {
    fn allocate(&mut self) {
        debug!("Allocating with Saba..");
        unimplemented!()
    }
}

/// Constructor for SabaAllocator.
impl<Sensitivity: Model> SabaAllocator<Sensitivity> {
    pub fn new() -> Self {
        SabaAllocator {
            sensitivity_table: HashMap::new(),
            allocation_queue: AllocationQueue::new(),
        }
    }
}

impl<Sensitivity: Model> SabaAllocator<Sensitivity> {
    fn get_performance(&self, app: &str, bw: f32) -> Option<f32> {
        match self.sensitivity_table.get(app) {
            Some(model) => Some(model.slowdown(bw)),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct AllocationJob {
    pub applications: Vec<String>,
}

#[derive(Debug)]
struct AllocationQueue {
    jobs: Mutex<Option<VecDeque<AllocationJob>>>,
    cvar: Condvar,
}

impl AllocationQueue {
    pub fn new() -> Self {
        AllocationQueue {
            jobs: Mutex::new(Some(VecDeque::new())),
            cvar: Condvar::new(),
        }
    }
}

impl AllocationQueue {
    pub fn allocate(&self, unallocated_applications: Vec<AllocationJob>) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(queue) = jobs.as_mut() {
            queue.extend(unallocated_applications);
            self.cvar.notify_all();
        }
    }
    pub fn wait_for_job(&self) -> Option<AllocationJob> {
        let mut jobs = self.jobs.lock().unwrap();
        loop {
            match jobs.as_mut()?.pop_front() {
                Some(job) => return Some(job),
                None => jobs = self.cvar.wait(jobs).unwrap(),
            }
        }
    }
    pub fn end(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        *jobs = None;
        self.cvar.notify_all();
    }
}
