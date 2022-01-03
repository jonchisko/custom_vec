use std::ptr::NonNull;
use std::alloc::{alloc, realloc, dealloc, Layout};

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl <T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn length(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");
        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Could not create a layout");
            // unsafe: allocation needs to be nonzero, layout > 0
            let ptr = unsafe { alloc(layout) as *mut T };
            let ptr = NonNull::new(ptr).expect("Could not allocate memory");
            // unsafe: ptr is non-null and we have just allocated enough
            // space for this item (and 3 more)
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
            }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2)
                .expect("Capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Cannot allocate");
            let ptr = unsafe {
                let layout = Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = realloc(
                    self.ptr.as_ptr() as *mut u8,
                    layout,
                    new_size
                );
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            Some(&* self.ptr.as_ptr().add(index))
        }
    }
}

impl <T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe{
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let size = std::mem::size_of::<T>() * self.len;
            let align = std::mem::align_of::<T>();
            let layout = Layout::from_size_align_unchecked(size, align);
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1usize);
        vec.push(2usize);
        vec.push(3usize);
        vec.push(4usize);
        vec.push(5usize);


        assert_eq!(*vec.get(3).unwrap(), 4usize);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.length(), 5);
    }
}