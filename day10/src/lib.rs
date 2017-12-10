pub const LOOP_SIZE: usize = 256;
type Element = u8;

pub struct LoopHash {
    elements: Vec<Element>,
    position: usize,
    skip: usize,
}

impl LoopHash {
    pub fn of_length(size: usize) -> LoopHash {
        assert!(size > 0);
        assert!(size - 1 <= Element::max_value() as usize);
        let mut elements = Vec::with_capacity(size);
        // ensure that this works if `size - 1 == Element::max_size()`
        elements.extend((0 as Element)..((size - 1) as Element));
        elements.push((size - 1) as Element);
        assert_eq!(elements.len(), size);
        LoopHash {
            elements: elements,
            position: 0,
            skip: 0,
        }
    }

    pub fn new() -> LoopHash {
        LoopHash::of_length(LOOP_SIZE)
    }

    pub fn twist<T>(&mut self, length: T)
    where
        T: Into<usize>,
    {
        let length: usize = length.into();
        self.reverse(length);
        self.position = (self.position + self.skip + length) % self.elements.len();
        self.skip += 1;
    }

    fn reverse(&mut self, length: usize) {
        for raw_index in 0..(length / 2) {
            let head_index = (self.position + raw_index) % self.elements.len();
            let tail_index = (self.elements.len() + self.position + length - raw_index - 1) %
                self.elements.len();
            self.elements.swap(head_index, tail_index);
        }
    }

    pub fn initial_product(&self) -> usize {
        self.elements[0] as usize * self.elements[1] as usize
    }

    pub fn twist_list<S, T>(&mut self, list: &S)
    where
        S: AsRef<[T]>,
        T: Copy + Into<usize>,
    {
        for &length in list.as_ref() {
            self.twist(length);
        }
    }
}

pub fn hash<S, T>(list: &S) -> String
where
    S: AsRef<[T]>,
    T: Copy + Into<usize>,
{
    let mut lh = LoopHash::new();
    for _ in 0..64 {
        lh.twist_list(list);
        lh.twist_list(&[17_u8, 31, 73, 47, 23]);
    }

    lh.elements
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0 as Element, |l, r| l ^ r))
        .map(|dense| format!("{:02x}", dense))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut lh = LoopHash::of_length(5);
        lh.twist(3_usize);
        assert_eq!(lh.elements, vec![2, 1, 0, 3, 4]);
        assert_eq!(lh.position, 3);
        assert_eq!(lh.skip, 1);
        lh.twist(4_usize);
        assert_eq!(lh.elements, vec![4, 3, 0, 1, 2]);
        assert_eq!(lh.position, 3);
        assert_eq!(lh.skip, 2);
        lh.twist(1_usize);
        assert_eq!(lh.elements, vec![4, 3, 0, 1, 2]);
        assert_eq!(lh.position, 1);
        assert_eq!(lh.skip, 3);
        lh.twist(5_usize);
        assert_eq!(lh.elements, vec![3, 4, 2, 1, 0]);
        assert_eq!(lh.position, 4);
        assert_eq!(lh.skip, 4);
    }

    #[test]
    fn test_example_compact() {
        let mut lh = LoopHash::of_length(5);
        lh.twist_list(&[3_usize, 4, 1, 5]);
        assert_eq!(lh.initial_product(), 12);
    }

    #[test]
    fn test_hash_examples() {
        let cases = [
            ("", "a2582a3a0e66e6e86e3812dcb672a272"),
            ("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"),
            ("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"),
            ("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"),
        ];
        for &(input, expected) in cases.iter() {
            assert_eq!(hash(&input.as_bytes()), expected);
        }
    }
}
