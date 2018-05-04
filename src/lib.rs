#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn system() {
        let s = System::new();
    }
}

mod numa;

extern crate errno;

use numa::*;
use std::collections::HashSet;

type BitSet = HashSet<u64>;

#[derive(Debug)]
pub struct CpuSet(BitSet);
#[derive(Debug)]
pub struct NodeSet(BitSet);

impl CpuSet {
    pub fn new() -> CpuSet {
        CpuSet(BitSet::new())
    }
}

impl IntoIterator for CpuSet {
    type Item = <BitSet as IntoIterator>::Item;
    type IntoIter = <BitSet as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for NodeSet {
    type Item = <BitSet as IntoIterator>::Item;
    type IntoIter = <BitSet as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<CpuMask> for CpuSet {
    fn from(m: CpuMask) -> CpuSet {
        let mut s = CpuSet::new();
        for i in 0..m.len() {
            if m.is_set(i) {
                s.0.insert(i as u64);
            }
        }
        s
    }
}


#[derive(Clone, Debug)]
enum ErrorKind {
    Unexpected,
    Errno,
}

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    fn new(kind: ErrorKind, message: &str) -> Error {
        Error {
            kind: kind,
            message: message.to_owned(),
        }
    }
}

impl From<errno::Errno> for Error {
    fn from(e: errno::Errno) -> Error {
        Error::new(ErrorKind::Errno, &format!("{}", e))
    }
}

type Result<T> = std::result::Result<T, Error>;

pub struct System {}

struct NodeMask {
    raw: *mut bitmask,
}

impl NodeMask {
    pub fn new() -> NodeMask {
        let raw = unsafe { numa_allocate_nodemask() };

        NodeMask { raw: raw }
    }

    pub fn set(&mut self, u: usize) {
        unsafe { numa_bitmask_setbit(self.raw, u as std::os::raw::c_uint) };
    }



    pub fn drop(&mut self) {
        unsafe { numa_bitmask_free(self.raw) }
    }
}

impl From<NodeSet> for NodeMask {
    fn from(s: NodeSet) -> NodeMask {
        let mut mask = NodeMask::new();

        for e in s {
            mask.set(e as usize);
        }

        mask
    }
}

impl From<*mut bitmask> for NodeMask {
    fn from(b: *mut bitmask) -> NodeMask {

        let new = NodeMask::new();

        unsafe {copy_bitmask_to_bitmask(b, new.raw)}

        new
    }
}

struct CpuMask {
    raw: *mut bitmask,
}

impl CpuMask {
    pub fn new() -> CpuMask {
        let raw = unsafe { numa_allocate_cpumask() };

        CpuMask { raw: raw }
    }

    pub fn set(&mut self, u: usize) {
        unsafe { numa_bitmask_setbit(self.raw, u as std::os::raw::c_uint) };
    }

    pub fn is_set(&self, u: usize) -> bool {
        match unsafe {numa_bitmask_isbitset(self.raw, u as u32)} {
            0 => false,
            _ => true
        }
    }

    pub fn len(&self) -> usize {
        8 * unsafe {numa_bitmask_nbytes(self.raw)} as usize
    }

    pub fn drop(&mut self) {
        unsafe { numa_bitmask_free(self.raw) }
    }
}

impl From<*mut bitmask> for CpuMask {
    fn from(b: *mut bitmask) -> CpuMask {

        let new = CpuMask::new();

        unsafe {copy_bitmask_to_bitmask(b, new.raw)}

        new
    }
}

impl System {
    pub fn new() -> System {
        System {}
    }

    pub fn is_available(&self) -> bool {
        match unsafe { numa_available() } {
            -1 => return false,
            0 => return true,
            _ => panic!("Unexpected"),
        }
    }

    pub fn all_cpus(&self) -> CpuSet {
        /*
        numa_all_cpus_ptr points to a bitmask that is allocated by the
       library with bits representing all cpus on which the calling task may
       execute.  This set may be up to all cpus on the system, or up to the
       cpus in the current cpuset.  The bitmask is allocated by a call to
       numa_allocate_cpumask() using size numa_num_possible_cpus().  The set
       of cpus to record is derived from /proc/self/status, field
       "Cpus_allowed".  The user should not alter this bitmask.
       */

      let owned = CpuMask::from( unsafe {numa_all_cpus_ptr} );
      CpuSet::from(owned)
    }

    pub fn run_on(&self, nodes: NodeSet) -> Option<Error> {
        /*
        numa_run_on_node() runs the current task and its children on a
       specific node. They will not migrate to CPUs of other nodes until the
       node affinity is reset with a new call to numa_run_on_node_mask().
       Passing -1 permits the kernel to schedule on all nodes again.  On
       success, 0 is returned; on error -1 is returned, and errno is set to
       indicate the error.
       */

        let mask = NodeMask::from(nodes);

        let res = unsafe { numa_run_on_node_mask(mask.raw) };

        match res {
            0 => None,
            -1 => Some(Error::from(errno::errno())),
            _ => Some(Error::new(ErrorKind::Unexpected, "numa_run_on_node_mask returned unexpected"))
        }
    }
}

/*

numa_allocate_nodemask() returns a bitmask of a size equal to the
       kernel's node mask (kernel type nodemask_t).  In other words, large
       enough to represent MAX_NUMNODES nodes.  This number of nodes can be
       gotten by calling numa_num_possible_nodes().  The bitmask is zero-
       filled.

*/

/*

numa_allocate_cpumask () returns a bitmask of a size equal to the
       kernel's cpu mask (kernel type cpumask_t).  In other words, large
       enough to represent NR_CPUS cpus.  This number of cpus can be gotten
       by calling numa_num_possible_cpus().  The bitmask is zero-filled.

*/
