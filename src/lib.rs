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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CornerPart {
    URF = 0,
    UFL = 1,
    ULB = 2,
    UBR = 3,
    DFR = 4,
    DLF = 5,
    DBL = 6,
    DRB = 7,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CornerOrientation {
    FRONT = 0,
    RIGHT = 1,
    LEFT = 2,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    corner_parts: [CornerPart; 8],
    corner_orientations: [CornerOrientation; 8],
    edge_parts:Vec<u8>,
    edge_orientations: Vec<u8>,
}

#[allow(dead_code)]
impl Cube {
    pub fn new(
        corner_parts: [CornerPart; 8],
        corner_orientations: [CornerOrientation; 8],
        edge_parts: Vec<u8>,
        edge_orientations: Vec<u8>,
    ) -> Self {
        Self { corner_parts, corner_orientations, edge_parts, edge_orientations }
    }

    fn is_solved(&self) -> bool {
        self.corner_parts == [
            CornerPart::URF,
            CornerPart::UFL,
            CornerPart::ULB,
            CornerPart::UBR,
            CornerPart::DFR,
            CornerPart::DLF,
            CornerPart::DBL,
            CornerPart::DRB,
        ]
            && self.corner_orientations.iter().all(|&x| x == CornerOrientation::FRONT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SERIAL_CORNER_PARTS: [CornerPart; 8] = [
        CornerPart::URF,
        CornerPart::UFL,
        CornerPart::ULB,
        CornerPart::UBR,
        CornerPart::DFR,
        CornerPart::DLF,
        CornerPart::DBL,
        CornerPart::DRB,
    ];

    #[test]
    fn cube_is_solved_should_return_true() {
        let cube = Cube::new(
            SERIAL_CORNER_PARTS,
            [CornerOrientation::FRONT; 8],
            vec![], vec![]
        );
        assert!(cube.is_solved());
    }

    #[test]
    fn cube_is_solved_should_return_false_when_corner_parts_is_not_serial_number() {
        let cube = Cube::new(
[
        CornerPart::ULB,
        CornerPart::UBR,
        CornerPart::DFR,
        CornerPart::DLF,
        CornerPart::DBL,
        CornerPart::DRB,
        CornerPart::URF,
        CornerPart::UFL,
    ],
            [CornerOrientation::FRONT; 8],
            vec![], vec![]
        );
        assert!(!cube.is_solved());
    }

    #[test]
    fn cube_is_solved_should_return_false_when_corner_orientations_is_not_front() {
        let cube = Cube::new(
            SERIAL_CORNER_PARTS,
            [CornerOrientation::LEFT; 8],
            vec![], vec![]
        );
        assert!(!cube.is_solved());
    }
}
