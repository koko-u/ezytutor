use std::ops::Deref;
use std::sync::Mutex;

use crate::model::courses::Course;

#[derive(Debug, Default)]
pub struct CoursesSate(Mutex<Vec<Course>>);

impl Deref for CoursesSate {
    type Target = Mutex<Vec<Course>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}