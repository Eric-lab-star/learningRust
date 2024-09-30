#[cfg(test)]
mod test {
    #[test]
    fn iteratro_demo() {
        let v1 = vec![1,2,3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iter_sum() {
        let v = vec![1,2,3];
        let iter_v = v.iter();
        let sum: i32 = iter_v.sum();
        assert_eq!(sum, 6);
    }
}
