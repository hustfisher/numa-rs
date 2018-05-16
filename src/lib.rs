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
mod node;
mod set;
mod mask;

extern crate errno;

use numa::*;
pub use set::{CpuSet, NodeSet};
pub use mask::{CpuMask, NodeMask};
pub use node::Node;



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

pub struct System {
    all_cpus: CpuSet,
    all_nodes: NodeSet,
}



impl System {
    pub fn new() -> System {

        let all_cpus = CpuSet::from(CpuMask::from( unsafe {numa_all_cpus_ptr} ));
        let all_nodes = NodeSet::from(NodeMask::from( unsafe {numa_all_nodes_ptr} ));

        System {
            all_cpus: all_cpus,
            all_nodes: all_nodes,
        }
    }

    pub fn is_available(&self) -> bool {
        match unsafe { numa_available() } {
            -1 => return false,
            0 => return true,
            _ => panic!("Unexpected"),
        }
    }

    pub fn all_cpus_ref(&self) -> &CpuSet {
      &self.all_cpus
    }

    pub fn all_nodes_ref(&self) -> &NodeSet {
      &self.all_nodes
    }

    pub fn run_on(&mut self, nodes: NodeSet) -> Result<()> {
        /*
        numa_run_on_node() runs the current task and its children on a
       specific node. They will not migrate to CPUs of other nodes until the
       node affinity is reset with a new call to numa_run_on_node_mask().
       Passing -1 permits the kernel to schedule on all nodes again.  On
       success, 0 is returned; on error -1 is returned, and errno is set to
       indicate the error.
       */

        let mut mask = NodeMask::from(nodes);

        let res = unsafe { numa_run_on_node_mask(mask.raw_mut()) };

        match res {
            0 => Ok(()),
            -1 => Err(Error::from(errno::errno())),
            _ => Err(Error::new(ErrorKind::Unexpected, "numa_run_on_node_mask returned unexpected"))
        }
    }

    /// get CPUs that the system is currently allowed to use
    pub fn run_cpus(&self) -> CpuSet {
        let mask = unsafe { numa_get_run_node_mask() };
        CpuSet::from(CpuMask::from(mask))
    }

    /// get numa nodes that the sysm is currently allowed to use
    pub fn run_nodes(&self) -> NodeSet {
        let mask = unsafe { numa_get_membind() };
        NodeSet::from(NodeMask::from(mask))
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
