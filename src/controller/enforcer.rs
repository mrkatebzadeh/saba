/* enforcer.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

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

/* enforcer.rs ends here */
