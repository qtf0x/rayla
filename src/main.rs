fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);

        std::assert_matches!(Point::try_from(&a), Ok(_));
        std::assert_matches!(Vector::try_from(&a), Err(_));
    }

    #[test]
    fn tuple_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);

        std::assert_matches!(Point::try_from(&a), Err(_));
        std::assert_matches!(Vector::try_from(&a), Ok(_));
    }

    #[test]
    fn create_a_point() {
        let p = Point::new(4.0, -4.0, 3.0);

        assert_eq!(Tuple::from(p), Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn create_a_vector() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(Tuple::from(v), Tuple::new(4.0, -4.0, 3.0, 0.0));
    }
}
