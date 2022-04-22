pub struct ArrayQueue<const SIZE: usize> {
    data: [u8; SIZE],
    first: usize,
    last: usize,
    count: usize,
}

impl<const SIZE: usize> ArrayQueue<SIZE> {
    pub const fn new() -> Self {
        ArrayQueue {
            data: [0; SIZE],
            first: 0,
            last: SIZE - 1,
            count: 0,
        }
    }

    pub fn enqueue(&mut self, item: u8) -> bool {
        if self.count == SIZE {
            return false;
        }
        self.last = (self.last + 1) % SIZE;
        self.data[self.last] = item;
        self.count += 1;
        return true;
    }

    pub fn dequeue(&mut self) -> Option<u8> {
        if self.count == 0 {
            return None;
        }
        let item = self.data[self.first];
        self.first = (self.first + 1) % SIZE;
        self.count -= 1;
        return Some(item);
    }
}
