use core::cell::UnsafeCell;
use core::marker::Sync;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct FakeMutex<T> {
    data: UnsafeCell<T>,
}

pub struct FakeMutexGuard<'a, T: 'a> {
    data: &'a mut T,
}

unsafe impl<T> Sync for FakeMutex<T> {}

impl<T> FakeMutex<T> {
    pub fn new(user_data: T) -> FakeMutex<T> {
        FakeMutex {
            data: UnsafeCell::new(user_data),
        }
    }

    pub fn lock(&self) -> FakeMutexGuard<T> {
        FakeMutexGuard {
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<'a, T> Deref for FakeMutexGuard<'a, T>{
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T { &*self.data }
}

impl<'a, T> DerefMut for FakeMutexGuard<'a, T>{
    fn deref_mut<'b>(&'b mut self) -> &'b mut T { &mut *self.data }
}