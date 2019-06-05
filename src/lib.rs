use std::cmp;

fn permutations<T: Clone + PartialEq + Ord>(mut vector: Vec<T>) -> Vec<Vec<T>> {
    let mut permutations = Vec::new();

    let n = {
        let mut vector = vector.to_vec();
        vector.sort();
        vector.dedup();
        vector.len()
    };

    let mut stack_state = vec![0; n];

    permutations.push(vector.to_vec());

    let mut idx = 0;
    while idx < n {
        if stack_state[idx] < idx {
            vector.swap(if idx % 2 == 0 { 0 } else { stack_state[idx] }, idx);
            permutations.push(vector.to_vec());
            stack_state[idx] += 1;
            idx = 0;
        } else {
            stack_state[idx] = 0;
            idx += 1;
        }
    }

    return permutations;
}

pub struct Package {
    crate_dimensions: Vec<usize>,
    box_dimensions: Vec<usize>
}

impl Package {
    pub fn new(crate_dimensions: Vec<usize>, box_dimensions: Vec<usize>) -> Self {
        let n = cmp::min(crate_dimensions.len(), box_dimensions.len());

        Package {
            crate_dimensions: (&crate_dimensions[0..n]).to_vec(),
            box_dimensions: (&box_dimensions[0..n]).to_vec()
        }
    }

    pub fn count_boxes(&self) -> usize {
        let Package { crate_dimensions, box_dimensions } = self;

        crate_dimensions.iter()
            .zip(box_dimensions.iter())
            .map(|(&crate_dimension, &box_dimension)| crate_dimension / box_dimension)
            .product()
    }

    pub fn count_boxes_by_rotating(&self) -> usize {
        permutations(self.box_dimensions.clone())
            .into_iter()
            .map(|box_dimensions| (Package { box_dimensions, crate_dimensions: self.crate_dimensions.to_vec() }).count_boxes())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_boxes() {
        // fit1
        assert_eq!(Package::new(vec![25, 18], vec![6, 5]).count_boxes(), 12);
        assert_eq!(Package::new(vec![10, 10], vec![1, 1]).count_boxes(), 100);
        assert_eq!(Package::new(vec![12, 34], vec![5, 6]).count_boxes(), 10);
        assert_eq!(Package::new(vec![12345, 678910], vec![1112, 1314]).count_boxes(), 5676);
        assert_eq!(Package::new(vec![5, 100], vec![6, 1]).count_boxes(), 0);
    }

    #[test]
    fn count_boxes_by_rotating() {
        // fit2
        assert_eq!(Package::new(vec![25, 18], vec![6, 5]).count_boxes_by_rotating(), 15);
        assert_eq!(Package::new(vec![12, 34], vec![5, 6]).count_boxes_by_rotating(), 12);
        assert_eq!(Package::new(vec![12345, 678910], vec![1112, 1314]).count_boxes_by_rotating(), 5676);
        assert_eq!(Package::new(vec![5, 5], vec![3, 2]).count_boxes_by_rotating(), 2);
        assert_eq!(Package::new(vec![5, 100], vec![6, 1]).count_boxes_by_rotating(), 80);
        assert_eq!(Package::new(vec![5, 5], vec![6, 1]).count_boxes_by_rotating(), 0);

        // fit3
        assert_eq!(Package::new(vec![10, 10, 10], vec![1, 1, 1]).count_boxes_by_rotating(), 1000);
        assert_eq!(Package::new(vec![12, 34, 56], vec![7, 8, 9]).count_boxes_by_rotating(), 32);
        assert_eq!(Package::new(vec![123, 456, 789], vec![10, 11, 12]).count_boxes_by_rotating(), 32604);
        assert_eq!(Package::new(vec![1234567, 89101112, 13141516], vec![171819, 202122, 232425]).count_boxes_by_rotating(), 174648);

        // fitn
        assert_eq!(Package::new(vec![3, 4], vec![1, 2]).count_boxes_by_rotating(), 6);
        assert_eq!(Package::new(vec![123, 456, 789], vec![10, 11, 12]).count_boxes_by_rotating(), 32604);
        assert_eq!(Package::new(vec![123, 456, 789, 1011, 1213, 1415], vec![16, 17, 18, 19, 20, 21]).count_boxes_by_rotating(), 1883443968);
    }
}
