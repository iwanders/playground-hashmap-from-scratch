#[repr(C)]
struct Wrapper {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn um_u64_u64_create() -> *mut Wrapper;
    fn um_u64_u64_free(w: *mut Wrapper);
    fn um_u64_u64_is_empty(w: *mut Wrapper) -> bool;
    fn um_u64_u64_insert(w: *mut Wrapper, key: u64, value: u64);
    fn um_u64_u64_contains(w: *mut Wrapper, key: u64) -> bool;
    fn um_u64_u64_get(w: *mut Wrapper, key: u64) -> u64;
    // fn um_u64_u64_remove(w: *mut Wrapper, key: u64);
    fn um_u64_u64_remove_return(w: *mut Wrapper, key: u64) -> u64;
    fn um_u64_u64_len(w: *mut Wrapper) -> u64;
    // fn bar_function(x: i32) -> i32;
}

pub struct CppStdUnorderedMapU64U64 {
    w: *mut Wrapper,
}
impl Drop for CppStdUnorderedMapU64U64 {
    fn drop(&mut self) {
        unsafe { um_u64_u64_free(self.w) };
    }
}

impl CppStdUnorderedMapU64U64 {
    pub fn new() -> Self {
        Self {
            w: unsafe { um_u64_u64_create() },
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { um_u64_u64_is_empty(self.w) }
    }
    pub fn insert(&mut self, key: u64, value: u64) {
        unsafe {
            um_u64_u64_insert(self.w, key, value);
        }
    }
    pub fn get(&self, key: &u64) -> Option<u64> {
        unsafe { Some(um_u64_u64_get(self.w, *key)) }
    }
    pub fn contains_key(&self, key: &u64) -> bool {
        unsafe { um_u64_u64_contains(self.w, *key) }
    }
    pub fn remove(&mut self, key: &u64) -> Option<u64> {
        unsafe { Some(um_u64_u64_remove_return(self.w, *key)) }
    }

    pub fn len(&self) -> usize {
        unsafe { um_u64_u64_len(self.w) as usize }
    }
}
