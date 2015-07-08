extern crate libc;

use std::ops;
use std::slice;

pub struct SecureBuffer {
  data: *mut u8, length: usize
}

impl SecureBuffer {
  pub fn new(length: usize) -> SecureBuffer {
    let data = unsafe { libc::calloc(length as libc::size_t, 1) as *mut u8 };

    if data.is_null() {
      panic!("Failed to allocate memory for SecureBuffer");
    }

    return SecureBuffer {
      data: data, length: length
    }
  }

  pub fn zero(&mut self) {
    for i in 0 .. self.length {
      self[i] = 0;
    }
  }
}

impl ops::Index<usize> for SecureBuffer {
  type Output = u8;

  #[inline]
  fn index(&self, index: usize) -> &u8 {
    &(**self)[index]
  }
}

impl ops::IndexMut<usize> for SecureBuffer {
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut u8 {
    &mut (**self)[index]
  }
}

impl ops::Index<ops::Range<usize>> for SecureBuffer {
  type Output = [u8];

  #[inline]
  fn index(&self, index: ops::Range<usize>) -> &[u8] {
    ops::Index::index(&**self, index)
  }
}

impl ops::Index<ops::RangeTo<usize>> for SecureBuffer {
  type Output = [u8];

  #[inline]
  fn index(&self, index: ops::RangeTo<usize>) -> &[u8] {
    ops::Index::index(&**self, index)
  }
}

impl ops::Index<ops::RangeFrom<usize>> for SecureBuffer {
  type Output = [u8];

  #[inline]
  fn index(&self, index: ops::RangeFrom<usize>) -> &[u8] {
    ops::Index::index(&**self, index)
  }
}

impl ops::Index<ops::RangeFull> for SecureBuffer {
  type Output = [u8];

  #[inline]
  fn index(&self, _index: ops::RangeFull) -> &[u8] {
    self
  }
}

impl ops::IndexMut<ops::Range<usize>> for SecureBuffer {
  #[inline]
  fn index_mut(&mut self, index: ops::Range<usize>) -> &mut [u8] {
    ops::IndexMut::index_mut(&mut **self, index)
  }
}

impl ops::IndexMut<ops::RangeTo<usize>> for SecureBuffer {
  #[inline]
  fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut [u8] {
    ops::IndexMut::index_mut(&mut **self, index)
  }
}

impl ops::IndexMut<ops::RangeFrom<usize>> for SecureBuffer {
  #[inline]
  fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut [u8] {
    ops::IndexMut::index_mut(&mut **self, index)
  }
}

impl ops::IndexMut<ops::RangeFull> for SecureBuffer {
  #[inline]
  fn index_mut(&mut self, _index: ops::RangeFull) -> &mut [u8] {
    self
  }
}

impl ops::Deref for SecureBuffer {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    return unsafe { slice::from_raw_parts(self.data, self.length) };
  }
}

impl ops::DerefMut for SecureBuffer {
  fn deref_mut(&mut self) -> &mut [u8] {
    return unsafe { slice::from_raw_parts_mut(self.data, self.length) };
  }
}

impl Drop for SecureBuffer {
  fn drop(&mut self) {
    self.zero();

    unsafe { libc::free(self.data as *mut libc::c_void) }
  }
}

#[test]
fn basic_functionality() {
  let mut b = SecureBuffer::new(4);

  b[0] = 1; b[1] = 2; b[3] = 4;

  assert_eq!(b.length, 4);

  assert_eq!(&b[..], &[1, 2, 0, 4]);

  b.zero();

  assert_eq!(&b[..], &[0, 0, 0, 0]);
}