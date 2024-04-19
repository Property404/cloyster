pub(crate) trait UsizeExt: Copy + Into<usize> + From<usize> {
    #[must_use]
    fn align_down(self, alignment: usize) -> usize {
        assert!(alignment.is_power_of_two());
        self.into() & !(alignment - 1)
    }

    #[must_use]
    fn is_aligned_to(self, alignment: usize) -> bool {
        assert!(alignment.is_power_of_two());
        self.align_down(alignment) == self.into()
    }

    #[must_use]
    fn align_up(self, alignment: usize) -> usize {
        assert!(alignment.is_power_of_two());
        if self.is_aligned_to(alignment) {
            self.into()
        } else {
            self.align_down(alignment) + alignment
        }
    }
}

impl UsizeExt for usize {}
