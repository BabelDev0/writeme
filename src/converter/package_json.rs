use serde_json::{json, Value};
use std::rc::Rc;

use anyhow::Error;

use super::{Component, Decorator};

// Concrete Decorators call the wrapped object and alter its result in some
// way.
struct PackageJson {
    component: Rc<dyn Component>,
}

impl Decorator for PackageJson {
    fn new(component: Rc<dyn Component>) -> Self {
        PackageJson { component }
    }
}

impl Component for PackageJson {
    fn convert(&self, file_contents: String) -> Result<Value, Error> {
        Ok(json!({}))
    }
}
