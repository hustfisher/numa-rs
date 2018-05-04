
use set::{CpuSet};
use mask::{CpuMask};
use numa::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    os_id: i32
}

impl Node {
    pub fn new(os_id: i32) -> Node {
        Node {
            os_id: os_id
        }
    }

    /// CPUs in this NUMA Node
    pub fn cpus(&self) -> CpuSet {

        let mut mask = CpuMask::new();
        match unsafe {numa_node_to_cpus(self.os_id, mask.raw_mut())} {
            0 => CpuSet::from(mask),
            _ => panic!("mask wasn't long enough?"),
        }

    }

    pub fn os_id(&self) -> i32 {
        self.os_id
    }
}