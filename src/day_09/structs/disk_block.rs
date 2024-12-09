use std::any::Any;

pub trait DiskBlock {
    fn as_any(&self) -> &dyn Any;
}

pub struct FileBlock {
    pub id: u32,
}

pub struct FreeSpace {}

impl DiskBlock for FileBlock {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl DiskBlock for FreeSpace {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
