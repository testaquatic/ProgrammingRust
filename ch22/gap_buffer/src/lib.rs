use std::{cmp::Ordering, ops::Range};

pub struct GapBuffer<T> {
    storage: Vec<T>,
    gap: Range<usize>,
}

impl<T> GapBuffer<T> {
    pub fn new() -> Self {
        GapBuffer {
            storage: Vec::new(),
            gap: 0..0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    pub fn len(&self) -> usize {
        self.capacity() - self.gap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn position(&self) -> usize {
        self.gap.start
    }

    /// # 주의
    /// `index`는 `self.storage`의 유효한 색인이어야 한다.
    unsafe fn space(&self, index: usize) -> *const T {
        self.storage.as_ptr().add(index)
    }

    /// # 주의
    /// `index`는 `self.storage`의 유효한 색인이어야 한다.
    unsafe fn space_mut(&mut self, index: usize) -> *mut T {
        self.storage.as_mut_ptr().add(index)
    }

    fn index_to_raw(&self, index: usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            index + self.gap.len()
        }
    }

    /// `index` 번째 요소의 레퍼런스를 반환한다.
    pub fn get(&self, index: usize) -> Option<&T> {
        let raw = self.index_to_raw(index);
        if raw < self.capacity() {
            unsafe { Some(&*self.space(raw)) }
        } else {
            None
        }
    }

    /// 현재 삽입 위치를 `pos`에 설정한다.
    pub fn set_position(&mut self, pos: usize) {
        if pos > self.len() {
            panic!("index {} out of range for GapBuffer", pos);
        }

        unsafe {
            let gap = self.gap.clone();
            match pos.cmp(&gap.start) {
                Ordering::Greater => {
                    let distance = pos - gap.start;
                    std::ptr::copy(self.space(gap.end), self.space_mut(gap.start), distance);
                }
                Ordering::Less => {
                    let distance = gap.start - pos;
                    std::ptr::copy(
                        self.space(pos),
                        self.space_mut(gap.end - distance),
                        distance,
                    );
                }
                _ => (),
            }

            self.gap = pos..pos + gap.len()
        }
    }

    pub fn insert(&mut self, elt: T) {
        if self.gap.is_empty() {
            self.enlarge_gap();
        }

        unsafe {
            let index = self.gap.start;
            std::ptr::write(self.space_mut(index), elt);
        }
        self.gap.start += 1;
    }

    // `Iter`가 산출하는 요소를 현재 삽입 위치에 넣고 삽입 위치는 그대로 놔둔다.
    pub fn insert_iter<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        iterable.into_iter().for_each(|item| self.insert(item));
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.gap.end == self.capacity() {
            return None;
        }

        let element = unsafe { std::ptr::read(self.space(self.gap.end)) };
        self.gap.end += 1;
        Some(element)
    }

    fn enlarge_gap(&mut self) {
        let mut new_capacity = self.capacity() * 2;
        if new_capacity == 0 {
            new_capacity = 4;
        }
        let mut new = Vec::with_capacity(new_capacity);
        let after_gap = self.capacity() - self.gap.end;
        let new_gap = self.gap.start..new.capacity() - after_gap;

        unsafe {
            std::ptr::copy_nonoverlapping(self.space(0), new.as_mut_ptr(), self.gap.start);
            let new_gap_end = new.as_mut_ptr().add(new_gap.end);
            std::ptr::copy_nonoverlapping(self.space(self.gap.end), new_gap_end, after_gap);
        }

        // 기존 Vec을 해제한다.
        self.storage = new;
        self.gap = new_gap;
    }
}

impl<T> Default for GapBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            (0..self.gap.start).for_each(|i| std::ptr::drop_in_place(self.space_mut(i)));
            (self.gap.end..self.capacity())
                .for_each(|i| std::ptr::drop_in_place(self.space_mut(i)));
        }
    }
}
