use crate::allocator::SabaAllocator;
use crate::enforcer::Enforcer;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Scheduler<'a> {
    allocator: &'a SabaAllocator,
    enforcer: &'a (dyn Enforcer + 'a),
}

impl<'a> Scheduler<'a> {
    #[allow(dead_code)]
    pub fn new(allocator: &'a SabaAllocator, enforcer: &'a dyn Enforcer) -> Self {
        Scheduler {
            allocator,
            enforcer,
        }
    }
}
