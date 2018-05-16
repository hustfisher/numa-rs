use numa::*;
use std;

use set::{NodeSet};
use std::ptr::NonNull;

pub struct NodeMask {
    raw: NonNull<bitmask>,
}

impl NodeMask {
    pub fn new() -> NodeMask {
        let raw = unsafe { numa_allocate_nodemask() };
        if let Some(nn) = NonNull::new(raw) {
            return NodeMask {
                raw: nn
            }
        } else {
            panic!();
        }
    }

    pub fn set(&mut self, u: usize) {
        let u = u as std::os::raw::c_uint;
        unsafe { numa_bitmask_setbit(self.raw.as_mut(), u) };
    }

    pub fn is_set(&self, u: usize) -> bool {
        let u = u as u32;
        let ptr = self.raw.as_ptr();
        match unsafe {numa_bitmask_isbitset(ptr, u)} {
            0 => false,
            _ => true
        }
    }

    pub fn len(&self) -> usize {
        let ptr = self.raw.as_ptr();
        8 * unsafe {numa_bitmask_nbytes(ptr)} as usize
    }

    pub fn drop(&mut self) {
        unsafe { numa_bitmask_free(self.raw.as_mut()) }
    }

    pub fn raw_mut(&mut self) -> &mut bitmask {
        unsafe {self.raw.as_mut()}
    }
}

impl From<NodeSet> for NodeMask {
    fn from(s: NodeSet) -> NodeMask {
        let mut mask = NodeMask::new();

        for e in s {
            mask.set(e.os_id() as usize);
        }

        mask
    }
}

impl From<*mut bitmask> for NodeMask {
    fn from(b: *mut bitmask) -> NodeMask {

        let mut new = NodeMask::new();

        unsafe {copy_bitmask_to_bitmask(b, new.raw_mut())}

        new
    }
}

pub struct CpuMask {
    raw: NonNull<bitmask>,
}

impl CpuMask {
    pub fn new() -> CpuMask {
        let raw = unsafe { numa_allocate_cpumask() };
        if let Some(nn) = NonNull::new(raw) {
            return CpuMask {
                raw: nn
            }
        } else {
            panic!();
        }
    }

    pub fn set(&mut self, u: usize) {
        let u = u as std::os::raw::c_uint;
        unsafe { numa_bitmask_setbit(self.raw.as_mut(), u) };
    }

    pub fn is_set(&self, u: usize) -> bool {
        let u = u as u32;
        let ptr = self.raw.as_ptr();
        match unsafe {numa_bitmask_isbitset(ptr, u)} {
            0 => false,
            _ => true
        }
    }

    pub fn len(&self) -> usize {
        let ptr = self.raw.as_ptr();
        8 * unsafe {numa_bitmask_nbytes(ptr)} as usize
    }

    pub fn drop(&mut self) {
        unsafe { numa_bitmask_free(self.raw.as_mut()) }
    }

    pub fn raw_mut(&mut self) -> &mut bitmask {
        unsafe {self.raw.as_mut()}
    }
}

impl From<*mut bitmask> for CpuMask {
    fn from(b: *mut bitmask) -> CpuMask {

        let mut new = CpuMask::new();

        unsafe {copy_bitmask_to_bitmask(b, new.raw_mut())}

        new
    }
}