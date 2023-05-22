//! This module provides all initialization-related functionality.

use crate::Window;
use std::marker::PhantomData;

/// All initialization-related things like texture/sounds/other-resources loading handled using it.
///
/// ### `PhantomData<&a mut Window>`Explanation
///
/// We dont really need to store reference to `Window` because `raylib` api is not object-oriented.
/// But we need compiler to make sure nobody other have referencec (mutable or not) to `Window` instance.
/// In other words, this trick lets compiler to ensure that only one thing goes in the same moment (resouce loading or drawing).
/// `std::marker::PhantomData` lets us to 'store' mutable reference to the `Window` instance from compiler's point of view
pub struct InitializationHandler<'a> {
    phantom: PhantomData<&'a mut Window>,
}

impl<'a> InitializationHandler<'a> {
    pub(crate) fn create(_window: &'a mut Window) -> Self {
        return InitializationHandler {
            phantom: PhantomData::default(),
        };
    }
    ///Load texture. Returns texture handler.
    pub fn load_texture(&mut self) {
        todo!()
    }
}

impl Drop for InitializationHandler<'_> {
    fn drop(&mut self) {
        todo!()
    }
}
