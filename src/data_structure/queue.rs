use std::{error::Error, io::prelude::*};

pub trait Queue<T> {
    fn enqueue(&mut self, x: T);
    fn dequeue(&mut self) -> T;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn len(&self) -> usize;
}

pub struct VecQueue<T> {
    items: Vec<T>,
}

impl<T> VecQueue<T> {
    pub fn new(capacity: usize) -> Self {
        VecQueue {
            items: Vec::with_capacity(capacity),
        }
    }
}

impl<T> Queue<T> for VecQueue<T> {
    fn enqueue(&mut self, x: T) {
        self.items.push(x);
    }
    fn dequeue(&mut self) -> T {
        self.items.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
    fn is_full(&self) -> bool {
        self.items.len() == self.len()
    }
    fn len(&self) -> usize {
        self.items.capacity()
    }
}

const ARRAY_QUEUE_SIZE: usize = 256;
use std::mem::MaybeUninit;
pub struct ArrayQueue<T> {
    items: [MaybeUninit<T>; ARRAY_QUEUE_SIZE],
    head: usize,
    tail: usize,
}

impl<T> ArrayQueue<T> {
    pub fn new() -> Self {
        Self {
            items: unsafe { MaybeUninit::uninit().assume_init() },
            head: 0,
            tail: 0,
        }
    }
}

impl<T> Queue<T> for ArrayQueue<T> {
    fn enqueue(&mut self, x: T) {
        let item = self.items.get_mut(self.tail).unwrap();
        self.tail += 1;

        unsafe {
            *item.as_mut_ptr() = x;
        }
    }
    fn dequeue(&mut self) -> T {
        let item = self.items.get_mut(self.head).unwrap();
        self.head += 1;

        let r: MaybeUninit<T> = std::mem::replace(item, MaybeUninit::uninit());
        unsafe { r.assume_init() }
    }
    fn is_empty(&self) -> bool {
        self.tail == self.head
    }
    fn is_full(&self) -> bool {
        self.tail >= ARRAY_QUEUE_SIZE
    }
    fn len(&self) -> usize {
        self.tail - self.head
    }
}

#[derive(Eq, PartialEq, Clone, Default, Hash)]
pub struct Process {
    name: String,
    time: usize,
    elapsed: usize,
}

impl Process {
    fn new(name: String, time: usize) -> Self {
        Self {
            name,
            time,
            elapsed: 0,
        }
    }
}

pub fn compute(queue: &mut impl Queue<Process>, quantom: usize) -> Vec<Process> {
    let mut result = vec![];
    let mut now_time: usize = 0;

    while !queue.is_empty() {
        now_time += quantom;
        let mut ps = queue.dequeue();

        match quantom.checked_sub(ps.time) {
            Some(n) => {
                now_time -= n;
                ps.elapsed = now_time;
                result.push(ps);
            }
            None => {
                ps.time -= quantom;
                queue.enqueue(ps);
            }
        };
    }

    result
}

pub fn input_queue(reader: &mut impl Read, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut lines = buf.lines();
    let mut head = lines.next().unwrap().split_whitespace();
    let len: usize = head.next().unwrap().parse()?;
    let quantom: usize = head.next().unwrap().parse()?;

    let mut queue = VecQueue::new(len);

    for line in lines {
        let mut s = line.split_whitespace();
        let p = Process::new(s.next().unwrap().into(), s.next().unwrap().parse()?);
        queue.enqueue(p);
    }

    let result = compute(&mut queue, quantom);
    for p in result {
        writeln!(writer, "{} {}", p.name, p.elapsed)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_vec_queue() {
        let mut q = VecQueue::new(3);
        q.enqueue(1);
        q.enqueue(10);
        q.enqueue(100);

        assert!(q.is_full());
        assert_eq!(1, q.dequeue());
        assert_eq!(10, q.dequeue());
        assert_eq!(100, q.dequeue());
        assert!(q.is_empty());
    }

    #[test]
    fn test_array_queue() {
        let mut q = ArrayQueue::new();
        q.enqueue(1);
        q.enqueue(10);
        q.enqueue(100);

        assert!(!q.is_full());
        assert_eq!(1, q.dequeue());
        assert_eq!(10, q.dequeue());
        assert_eq!(100, q.dequeue());
        assert!(q.is_empty());
    }

    #[test]
    fn test1() {
        let input = ["5 100", "p1 150", "p2 80", "p3 200", "p4 350", "p5 20"].join("\n");
        let mut writer = vec![];

        let result = input_queue(&mut input.as_bytes(), &mut writer);

        assert!(result.is_ok());
        let output = String::from_utf8(writer).unwrap();

        assert_eq!(
            output,
            vec!["p2 180", "p5 400", "p1 450", "p3 550", "p4 800", ""].join("\n")
        )
    }
}
