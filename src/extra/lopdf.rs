//! Add functionality missing from `lopdf`.

use lopdf::Document;
use lopdf::{Object,ObjectId};

/// It is not possible to access objects mutably through the provided methods on `Document`. Rather
/// than doing so using the struct fields and `BTreeMap` functions, this trait adds a new method
/// which is similar to `get_object`.
pub trait GetObjectMut {
    fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object>;
}

/// This (working) implementation is very similar to that of `get_object` and the changes
/// can be applied upstream. It may be possible to use the `Borrow` trait to write a single
/// function which can handle both mutable and immutable references.
impl GetObjectMut for Document {
    /// Get mutable object by object id, will recursively dereference a referenced object.
    fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        let is_ref;

        if let Some(object) = self.objects.get(&id) {
            is_ref = object.as_reference();
        } else {
            return None
        }

        if let Some(id) = is_ref {
            return self.get_object_mut(id);
        } else {
            return self.objects.get_mut(&id);
        }
    }
}
