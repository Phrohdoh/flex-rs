mod flex_ll;

type RawFlexItem = *mut flex_ll::flex_item;

#[derive(Debug)]
pub struct FlexItem {
    raw: RawFlexItem,
}

impl FlexItem {
    pub fn new() -> Self {
        // TODO: The current impl of flex (6aa0b654902083117a1ba175139d5a5e68ba3710) ensures that ptr will not be null.
        //       Consider trapping the abort that happens and returning `Option<Self>`.
        let ptr = unsafe { flex_ll::flex_item_new() };
        Self { raw: ptr }
    }

    pub fn add_child(&mut self, child: Self) {
        unsafe { flex_ll::flex_item_add(self.raw, child.raw) }

        // We don't ever want to call `drop(child)`.
        ::std::mem::forget(child);
    }

    pub fn child_count(&self) -> u32 {
        unsafe { flex_ll::flex_item_count(self.raw) }
    }
}

impl Drop for FlexItem {
    fn drop(&mut self) {
        unsafe { flex_ll::flex_item_free(self.raw); }
    }
}