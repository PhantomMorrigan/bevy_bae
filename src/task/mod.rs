pub mod compound;
pub mod primitive;

pub enum TaskStatus {
    Continue,
    Success,
    Failure,
}
