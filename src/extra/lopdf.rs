use lopdf::Document;
use lopdf::{Object,ObjectId};

pub trait GetObjectMut {
    fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object>;
}

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
