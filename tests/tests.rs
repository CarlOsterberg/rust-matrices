use rust_matrices::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let array: [[i32; 1]; 3] = [
            [i32::default()],
            [i32::default()],
            [i32::default()]
        ];
        let matrix = Matrix::from_array(array);
        let m: Matrix<i32, 3, 1> = Matrix::new();
        assert_eq!(m, matrix);
    }

    #[test]
    fn add() {

        let m1 = Matrix::from_array([
            [1],
            [2],
            [3]
        ]);
        let m2 = Matrix::from_array([
            [3],
            [4],
            [5]
        ]);
        let m3 = Matrix::from_array([
            [4],
            [6],
            [8]
        ]);
        assert_eq!(m1 + m2, m3);
        assert_eq!(m2 + m1, m3); // Associative
    }

    #[test]
    fn sub() {

        let m1 = Matrix::from_array([
            [3],
            [4],
            [5]
        ]);
        let m2 = Matrix::from_array([
            [1],
            [2],
            [3]
        ]);
        let m3 = Matrix::from_array([
            [2],
            [2],
            [2]
        ]);
        assert_eq!(m1 - m2, m3);
        assert_ne!(m2 - m1, m3); // Not associative
    }


    #[test]
    fn scalar() {

        let m = Matrix::from_array([
            [1,2,3],
            [4,5,6],
            [7,8,9]
        ]);
        let m2 = Matrix::from_array([
            [3,6,9],
            [12,15,18],
            [21,24,27]
        ]);
        assert_eq!(m.clone() * 3, m2); // Right hand side impl
        assert_eq!(3 * m, m2); // Left hand side impl
    }

    #[test]
    fn transpose() {

        let m1 = Matrix::from_array([
            [1, 2, 3],
            [0, -6, 7],
        ]);
        let m2 = Matrix::from_array([
            [1, 0],
            [2, -6],
            [3, 7],
        ]);
        assert_eq!(m1.transpose(), m2);
        assert_eq!(m1, m2.transpose());
        assert_eq!(m1.transpose().transpose(), m1);
    }

    #[test]
    fn multiplication() {

        let a = Matrix::from_array([
            [2, 3, 4],
            [1, 0, 0],
        ]);
        let b = Matrix::from_array([
            [0, 1000],
            [1, 100],
            [0, 10],
        ]);
        let ab = Matrix::from_array([
            [3, 2340],
            [0, 1000],
        ]);
        assert_eq!(a * b, ab);
        let c = Matrix::from_array([
            [1, 2],
            [3, 4],
        ]);
        let d = Matrix::from_array([
            [0, 1],
            [0, 0],
        ]);
        let e = Matrix::from_array([
            [0, 1],
            [0, 3],
        ]);
        let f = Matrix::from_array([
            [3, 4],
            [0, 0],
        ]);
        assert_eq!(c * d, e);
        assert_ne!(d * c, e);
        assert_eq!(d * c, f);
        assert_ne!(c * d, f);
    }

    #[test]
    fn get() {

        let a = Matrix::from_array([
            [2, 3, 4],
            [1, 0, 0],
        ]);

        assert_eq!(a.get(0, 0), 2);
        assert_eq!(a.get(0, 1), 3);
        assert_eq!(a.get(0, 2), 4);
        assert_eq!(a.get(1, 0), 1);
        assert_eq!(a.get(1, 1), 0);
        assert_eq!(a.get(1, 2), 0);
    }
}
