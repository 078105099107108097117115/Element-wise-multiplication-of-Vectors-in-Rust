pub mod elem_wise_mul {

    use std::ops::Mul;

    pub fn elem_wise_multiply<T: Copy>(vec1: &[T], vec2: &[T]) -> Option<Vec<T>>
    where
        T: Mul<Output=T>,
    {
        match vec1.len() == vec2.len() {
            true => Some(
                vec1.iter()
                    .zip(vec2)
                    .map(|(&x, &y)| x * y)
                    .collect::<Vec<T>>(),
            ),
            false => None,
        }
    }
}
