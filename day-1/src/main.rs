use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct FixedCapacityOrderedVec<T, P>
where
    T: PartialOrd + Ord + Copy,
    P: FnOnce(T, T) -> bool + Copy,
{
    v: Vec<T>,
    capacity: usize,
    pred: P,
}

impl<T, P> FixedCapacityOrderedVec<T, P>
where
    T: PartialOrd + Ord + Copy,
    P: FnOnce(T, T) -> bool + Copy,
{
    fn new(capacity: usize, pred: P) -> Self {
        FixedCapacityOrderedVec {
            v: Vec::new(),
            capacity,
            pred,
        }
    }

    fn insert(&mut self, value: T) {
        let partition = self
            .v
            .partition_point(|item| (self.pred)(item.to_owned(), value));
        self.v.insert(partition, value);
        self.v.truncate(self.capacity);
    }

    fn vec(&self) -> &Vec<T> {
        &self.v
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("input")?;
    {
        let mut top_n_max = FixedCapacityOrderedVec::new(3, |lhs: i32, rhs: i32| lhs > rhs);
        let reader = BufReader::new(input);
        let mut running_total_calories = 0;

        for line in reader.lines() {
            let s = line?;
            if s.is_empty() {
                if let Some(min_of_max_calories) = top_n_max.v.last() {
                    if running_total_calories > *min_of_max_calories {
                        top_n_max.insert(running_total_calories);
                    }
                } else {
                    top_n_max.insert(running_total_calories);
                }
                running_total_calories = 0;
            } else {
                let calories = s.parse::<i32>()?;
                running_total_calories += calories;
            }
        }

        let min_of_max_calories = top_n_max.v.last().unwrap().to_owned();
        if running_total_calories > min_of_max_calories {
            top_n_max.insert(running_total_calories);
        }

        println!(
            "{:?} = {}",
            top_n_max.vec(),
            top_n_max.vec().iter().sum::<i32>()
        );
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::FixedCapacityOrderedVec;

    #[test]
    fn construct() {
        let _ = FixedCapacityOrderedVec::new(5, |_: i32, _| true);
    }

    #[test]
    fn insert_1() {
        let p = |lhs, rhs| lhs < rhs;
        let mut v = FixedCapacityOrderedVec::new(5, p);
        v.insert(1);

        let vec = v.vec();
        assert_eq!(vec, &vec![1]);
    }

    #[test]
    fn insert_many() {
        let p = |lhs, rhs| lhs < rhs;
        let mut v = FixedCapacityOrderedVec::new(5, p);
        v.insert(1);
        v.insert(5);
        v.insert(4);
        v.insert(2);
        v.insert(3);

        let vec = v.vec();
        assert_eq!(vec, &vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn insert_many_truncated() {
        let p = |lhs, rhs| lhs < rhs;
        let mut v = FixedCapacityOrderedVec::new(3, p);
        v.insert(1);
        v.insert(5);
        v.insert(4);
        v.insert(2);
        v.insert(3);

        let vec = v.vec();
        assert_eq!(vec, &vec![1, 2, 3]);
    }
}
