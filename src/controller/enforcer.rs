use crate::allocator::AppAllocation;
use saba::clustering::QueueAssignment;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct EnforcementPlan {
    pub queue_assignments: Vec<QueueAssignment>,
    pub app_weights: Vec<AppAllocation>,
}

pub trait Enforcer: Debug + Send {
    fn enforce(&mut self, plan: &EnforcementPlan);
}

#[derive(Debug, Default)]
pub struct MockSwitchEnforcer;

impl Enforcer for MockSwitchEnforcer {
    fn enforce(&mut self, _plan: &EnforcementPlan) {}
}
