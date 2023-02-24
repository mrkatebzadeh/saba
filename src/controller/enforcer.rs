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
pub struct MockSwitchEnforcer {
    last_plan: Option<EnforcementPlan>,
}

impl MockSwitchEnforcer {
    pub fn last_plan(&self) -> Option<&EnforcementPlan> {
        self.last_plan.as_ref()
    }
}

impl Enforcer for MockSwitchEnforcer {
    fn enforce(&mut self, plan: &EnforcementPlan) {
        self.last_plan = Some(plan.clone());
    }
}
