use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Range {
    min: u32,
    max: u32,
}

impl Range {
    pub fn new(min: u32, max: u32) -> Result<Self, Error> {
        if min > max {
            Err(Error::General(
                "Max < Min in creation of Range value".into(),
            ))
        } else {
            Ok(Self { min, max })
        }
    }

    pub fn from_vector(vec: &[u32]) -> Result<Self, Error> {
        if vec.len() == 2 {
            if vec[0] > vec[1] {
                Err(Error::General(
                    "Max < Min in creation of Range value".into(),
                ))
            } else {
                Ok(Self {
                    min: vec[0],
                    max: vec[1],
                })
            }
        } else {
            Err(Error::General(
                "Creating Range with vector that has length not equal to 2".into(),
            ))
        }
    }

    pub fn new_range_vector(vec: &[Vec<u32>]) -> Result<Vec<Self>, Error> {
        let mut output = Vec::new();
        for range in vec.iter() {
            let new_val = Self::from_vector(range)?;
            output.push(new_val);
        }
        Ok(output)
    }

    pub fn contains(&self, val: u32) -> bool {
        val >= self.min && val <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_basic() -> Result<(), Error> {
        let range1 = Range::new(15, 30)?;
        assert_eq!(range1.min, 15);
        assert_eq!(range1.max, 30);
        Ok(())
    }

    #[test]
    fn invalid_range() {
        let range1 = Range::new(35, 20);
        assert!(range1.is_err());
        let err = range1.unwrap_err();
        assert!(matches!(err, Error::General(_)));
    }

    #[test]
    fn vec_of_ranges() -> Result<(), Error> {
        let input = vec![
            vec![0, 5],
            vec![4, 10],
            vec![8, 16],
            vec![17, 24],
            vec![21, 30],
        ];
        let ranges = Range::new_range_vector(&input)?;
        assert_eq!(ranges.len(), 5);
        assert_eq!(ranges[0], Range { min: 0, max: 5 });
        assert_eq!(ranges[1], Range { min: 4, max: 10 });
        assert_eq!(ranges[2], Range { min: 8, max: 16 });
        assert_eq!(ranges[3], Range { min: 17, max: 24 });
        assert_eq!(ranges[4], Range { min: 21, max: 30 });
        assert!(ranges[0].contains(4));
        assert!(!ranges[0].contains(8));
        assert!(ranges[3].contains(17));
        assert!(!ranges[3].contains(4));
        Ok(())
    }
}
