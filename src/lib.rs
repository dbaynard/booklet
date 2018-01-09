#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lopdf;

extern crate num;
#[macro_use]
extern crate num_derive;

pub mod reorder;
pub use reorder::*;

pub mod rearrange;
pub use rearrange::*;

mod extra {
    pub mod error {
        use std::io;
        use std::io::ErrorKind::*;

        pub fn nonzero_error() -> io::Error {
            io::Error::new(InvalidInput, "Need nonzero document length")
        }

        pub fn invalid(err: &str) -> io::Error {
            io::Error::new(InvalidData, err)
        }
    }

    pub mod lopdf {
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
    }
}
