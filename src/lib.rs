/// Up, Front, Left, Right, Back, Down
///          + - - - +
///          | * * * |
///          | * U * |
///          | * * * |
///  + - - - + - - - + - - - + - - - +
///  | * * * | * * * | * * * | * * * |
///  | * L * | * F * | * R * | * B * |
///  | * * * | * * * | * * * | * * * |
///  + - - - + - - - + - - - + - - - +
///          | * * * |
///          | * D * |
///          | * * * |
///          + - - - +
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    corner_parts: [u8; 8],
    corner_orientations: [u8; 8],
}

type Move = Cube;

#[allow(dead_code)]
impl Cube {
    pub fn new(corner_parts: [u8; 8], corner_orientations: [u8; 8]) -> Self {
        Self {
            corner_parts,
            corner_orientations,
        }
    }

    pub fn is_solved(&self) -> bool {
        self.corner_parts == [0, 1, 2, 3, 4, 5, 6, 7]
            && self.corner_orientations.iter().all(|&x| x == 0)
    }

    fn apply(self, move_state: &Move) -> Self {
        let corner_parts: [u8; 8] = move_state
            .corner_parts
            .iter()
            .map(|&x| self.corner_parts[x as usize])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let corner_orientations: [u8; 8] = move_state
            .corner_parts
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let a = self.corner_orientations[x as usize] as usize;
                let b = move_state.corner_orientations[i] as usize;
                let orientation = (a + b) % 3;
                orientation as u8
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Self {
            corner_parts,
            corner_orientations,
        }
    }

    pub fn right(self) -> Self {
        let right_move = Move::new([0, 5, 1, 3, 4, 6, 2, 7], [0, 2, 1, 0, 0, 1, 2, 0]);
        self.apply(&right_move)
    }

    pub fn right2(self) -> Self {
        self.right().right()
    }

    pub fn right_prime(self) -> Self {
        self.right().right().right()
    }

    pub fn left(self) -> Self {
        self.right_prime()
    }
}

#[cfg(test)]
mod tests_2x2 {
    use super::*;

    const SERIAL_CORNER_PARTS: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    #[test]
    fn cube_is_solved_should_return_true() {
        let cube = Cube::new(SERIAL_CORNER_PARTS, [0; 8]);
        assert!(cube.is_solved());
    }

    #[test]
    fn cube_is_solved_should_return_false_when_corner_parts_is_not_serial_number() {
        let cube = Cube::new([1, 2, 3, 4, 5, 6, 7, 0], [0; 8]);
        assert!(!cube.is_solved());
    }

    #[test]
    fn cube_is_solved_should_return_false_when_corner_orientations_is_not_front() {
        let cube = Cube::new(SERIAL_CORNER_PARTS, [1; 8]);
        assert!(!cube.is_solved());
    }

    #[test]
    fn right_move_should_return_right_move_cube() {
        let cube = Cube::new(SERIAL_CORNER_PARTS, [0; 8]);
        let right_move_cube = Cube::new([0, 5, 1, 3, 4, 6, 2, 7], [0, 2, 1, 0, 0, 1, 2, 0]);
        assert_eq!(cube.right(), right_move_cube);
    }

    #[test]
    fn right2_move_should_return_right2_move_cube() {
        let cube = Cube::new(SERIAL_CORNER_PARTS, [0; 8]);
        let right2_move_cube = Cube::new([0, 6, 5, 3, 4, 2, 1, 7], [0; 8]);
        assert_eq!(cube.right2(), right2_move_cube);
    }

    #[test]
    fn right_prime_move_should_return_right_prime_move_cube() {
        let cube = Cube::new(SERIAL_CORNER_PARTS, [0; 8]);
        let right_prime_move_cube = Cube::new([0, 2, 6, 3, 4, 1, 5, 7], [0, 2, 1, 0, 0, 1, 2, 0]);
        assert_eq!(cube.right_prime(), right_prime_move_cube);
    }

    #[test]
    fn left_move_should_return_left_move_cube() {
        let cube = Cube::new(SERIAL_CORNER_PARTS, [0; 8]);
        let left_move_cube = Cube::new([0, 2, 6, 3, 4, 1, 5, 7], [0, 2, 1, 0, 0, 1, 2, 0]);
        assert_eq!(cube.left(), left_move_cube);
    }
}
