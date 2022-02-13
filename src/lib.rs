mod arpx_job;
mod generic;

mod prelude {
    pub use crate::generic::*;
}

pub use crate::arpx_job::{job, task, Job, Process, Task};
