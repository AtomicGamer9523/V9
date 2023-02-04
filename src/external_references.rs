use crate::support::intptr_t;
use crate::AccessorNameGetterCallback;
use crate::FunctionCallback;
use crate::MessageCallback;
use core::ffi::c_void;
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub union ExternalReference<'s> {
  pub function: FunctionCallback,
  pub getter: AccessorNameGetterCallback<'s>,
  pub message: MessageCallback,
  pub pointer: *mut c_void,
}

#[derive(Debug, Clone)]
pub struct ExternalReferences {
  null_terminated: Vec<intptr_t>,
}

unsafe impl Sync for ExternalReferences {}

impl ExternalReferences {
  #[inline(always)]
  pub fn new(refs: &[ExternalReference]) -> Self {
    let null_terminated = refs
      .iter()
      .map(|&r| unsafe { core::mem::transmute(r) })
      .chain(core::iter::once(0)) // Add null terminator.
      .collect::<Vec<intptr_t>>();
    Self { null_terminated }
  }

  #[inline(always)]
  pub fn as_ptr(&self) -> *const intptr_t {
    self.null_terminated.as_ptr()
  }
}

impl core::ops::Deref for ExternalReferences {
  type Target = [intptr_t];
  fn deref(&self) -> &Self::Target {
    &self.null_terminated
  }
}

impl core::borrow::Borrow<[intptr_t]> for ExternalReferences {
  fn borrow(&self) -> &[intptr_t] {
    self
  }
}
