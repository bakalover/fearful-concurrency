use super::{core::Mutex, guard::Guard};
use std::ops::Deref;

/// Simple Reference associated with inner resource, that may be not measurable.
/// `Ar` must contain Guard item that will keep resource safe until reference will be dropped.
/// For that reason this item should have same lifetime.
/// Dropping `Ar` causes dropping Guard, that releases inner resource.
pub struct Ar<'a, R>
where
    R: ?Sized + 'a,
{
    resource_ref: &'a R,
    _guard: Guard<'a>,
}

/// Dereferencing `Ar` provides protected resource.
impl<'a, R> Deref for Ar<'a, R>
where
    R: ?Sized + 'a,
{
    type Target = R;
    fn deref(&self) -> &Self::Target {
        self.resource_ref
    }
}

/// Provider for `Ar`. Has the same lifetime as resource and consequently as mutex.
pub struct Rp<'a, R>
where
    R: ?Sized + 'a,
{
    resource: &'a R,
    mutex: Mutex,
}

impl<'a, R> Rp<'a, R>
where
    R: ?Sized + 'a,
{
    /// Provider binds resource's reference to mutex
    pub fn create_from(resource: &'a R) -> Self {
        Rp {
            resource,
            mutex: Mutex::new(),
        }
    }

    ///Acquiring provides unique `Ar` for first thread. Other should wait until reference is destroyed.
    pub fn acquire(&'a self) -> Ar<'a, R> {
        Ar {
            resource_ref: self.resource,
            _guard: Guard::new(&self.mutex),
        }
    }
}
