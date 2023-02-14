use crate::allocator::Allocator;
use crate::enforcer::Enforcer;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Scheduler<'a> {
    allocator: &'a (dyn Allocator + 'a),
    enforcer: &'a (dyn Enforcer + 'a),
}

impl<'a> Scheduler<'a> {
    #[allow(dead_code)]
    pub fn new(allocator: &'a dyn Allocator, enforcer: &'a dyn Enforcer) -> Self {
        Scheduler {
            allocator,
            enforcer,
        }
    }
}
