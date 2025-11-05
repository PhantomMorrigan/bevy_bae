pub mod compound;
pub mod relationship;
pub mod step;

pub enum TaskStatus {
    Continue,
    Success,
    Failure,
}
