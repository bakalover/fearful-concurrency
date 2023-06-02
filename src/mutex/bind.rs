use super::{core::Mutex, guard::Guard};
use std::ops::Deref;

/// Simple Reference associated with inner resource, that may be not measurable.
/// AtomRef must contain Guard item that will keep resource safe until reference will be dropped.
/// For that reason this item should have same lifetime.
/// Dropping AtomRef causes dropping Guard, that releases inner resource.
pub struct AtomRef<'a, R>
where
    R: ?Sized + 'a,
{
    resource_ref: &'a R,
    guard: Guard<'a>,
}

/// Dereferencing AtomRef provides protected resource.
impl<'a, R> Deref for AtomRef<'a, R>
where
    R: ?Sized + 'a,
{
    type Target = R;
    fn deref(&self) -> &Self::Target {
        self.resource_ref
    }
}

/// Provider for AtomRef. Has same lifetime as resource and consequently as mutex.
pub struct AtomRefProvider<'a, R>
where
    R: ?Sized + 'a,
{
    resource: &'a R,
    mutex: Mutex,
}

impl<'a, R> AtomRefProvider<'a, R>
where
    R: ?Sized + 'a,
{
    /// Provider binds resource reference to mutex
    pub fn create(resource: &'a R) -> Self {
        AtomRefProvider {
            resource: resource,
            mutex: Mutex::new(),
        }
    }

    ///Acquiring provides unique AtomRef for first thread. Other should wait until reference is destroyed.
    pub fn acquire(&'a self) -> AtomRef<'a, R> {
        AtomRef {
            resource_ref: &self.resource,
            guard: Guard::new(&self.mutex),
        }
    }
}
