#[derive(Debug)]
pub struct ObjectHeader {
    pub class_id: u32,
}

#[derive(Debug)]
pub struct JavaObject {
    pub header: ObjectHeader,
    pub fields: Vec<i64>,
}

impl JavaObject {
    pub fn new(class_id: u32, fields: Vec<i64>) -> Self {
        Self { header: ObjectHeader { class_id }, fields }
    }
}
