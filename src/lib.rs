#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CornerPart {
    ULB,
    URB,
    URF,
    ULF,
    DLB,
    DRB,
    DRF,
    DLF,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CornerOrientation {
    FRONT = 0,
    RIGHT = 1,
    LEFT = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EdgePart {
    LB,
    BR,
    RF,
    FL,
    UB,
    UR,
    UF,
    UL,
    DB,
    DR,
    DF,
    DL,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EdgeOrientation {
    FRONT = 0,
    TURNED = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
    cp: [CornerPart; 8],
    co: [CornerOrientation; 8],
    ep: [EdgePart; 12],
    eo: [EdgeOrientation; 12],
}

const MOVE_R: Move = Move {
    cp: [CornerPart::ULB, CornerPart::URB, CornerPart::URF, CornerPart::ULF, CornerPart::DLB, CornerPart::DRB, CornerPart::DRF, CornerPart::DLF],
    co: [CornerOrientation::FRONT; 8],
    ep: [EdgePart::LB, EdgePart::BR, EdgePart::RF, EdgePart::FL, EdgePart::UB, EdgePart::UR, EdgePart::UF, EdgePart::UL, EdgePart::DB, EdgePart::DR, EdgePart::DF, EdgePart::DL],
    eo: [EdgeOrientation::FRONT; 12],
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cube {
    cp: [CornerPart; 8],
    co: [CornerOrientation; 8],
    ep: [EdgePart; 12],
    eo: [EdgeOrientation; 12],
}

#[allow(dead_code)]
impl Cube {
    fn new(cp: [CornerPart; 8], co: [CornerOrientation; 8], ep: [EdgePart; 12], eo: [EdgeOrientation; 12]) -> Cube {
        Cube { cp, co, ep, eo }
    }

    fn solved() -> Self {
        Cube {
            cp : [CornerPart::ULB, CornerPart::URB, CornerPart::URF, CornerPart::ULF, CornerPart::DLB, CornerPart::DRB, CornerPart::DRF, CornerPart::DLF],
            co : [CornerOrientation::FRONT; 8],
            ep : [EdgePart::LB, EdgePart::BR, EdgePart::RF, EdgePart::FL, EdgePart::UB, EdgePart::UR, EdgePart::UF, EdgePart::UL, EdgePart::DB, EdgePart::DR, EdgePart::DF, EdgePart::DL],
            eo : [EdgeOrientation::FRONT; 12],
        }
    }

    fn is_solved(&self) -> bool {
        self.cp == [CornerPart::ULB, CornerPart::URB, CornerPart::URF, CornerPart::ULF, CornerPart::DLB, CornerPart::DRB, CornerPart::DRF, CornerPart::DLF] &&
        self.co == [CornerOrientation::FRONT; 8] &&
        self.ep == [EdgePart::LB, EdgePart::BR, EdgePart::RF, EdgePart::FL, EdgePart::UB, EdgePart::UR, EdgePart::UF, EdgePart::UL, EdgePart::DB, EdgePart::DR, EdgePart::DF, EdgePart::DL] &&
        self.eo == [EdgeOrientation::FRONT; 12]
    }

    fn apply_move(&mut self, m: Move) {
        self.cp = m.cp.map(|x| self.cp[x as usize]);
        self.co = m.co.iter().enumerate().map(|(i, x)| {

            match (x, self.co[i]) {
                (CornerOrientation::FRONT, CornerOrientation::FRONT) => CornerOrientation::FRONT,
                (CornerOrientation::FRONT, CornerOrientation::RIGHT) => CornerOrientation::RIGHT,
                (CornerOrientation::FRONT, CornerOrientation::LEFT) => CornerOrientation::LEFT,
                (CornerOrientation::RIGHT, CornerOrientation::FRONT) => CornerOrientation::RIGHT,
                (CornerOrientation::RIGHT, CornerOrientation::RIGHT) => CornerOrientation::LEFT,
                (CornerOrientation::RIGHT, CornerOrientation::LEFT) => CornerOrientation::FRONT,
                (CornerOrientation::LEFT, CornerOrientation::FRONT) => CornerOrientation::LEFT,
                (CornerOrientation::LEFT, CornerOrientation::RIGHT) => CornerOrientation::FRONT,
                (CornerOrientation::LEFT, CornerOrientation::LEFT) => CornerOrientation::RIGHT,
            }
        }).collect::<Vec<CornerOrientation>>().try_into().unwrap();
        self.ep = m.ep.map(|x| self.ep[x as usize]);
        self.eo = m.eo.iter().enumerate().map(|(i, x)| {

            match (x, self.eo[i]) {
                (EdgeOrientation::FRONT, EdgeOrientation::FRONT) => EdgeOrientation::FRONT,
                (EdgeOrientation::FRONT, EdgeOrientation::TURNED) => EdgeOrientation::TURNED,
                (EdgeOrientation::TURNED, EdgeOrientation::FRONT) => EdgeOrientation::TURNED,
                (EdgeOrientation::TURNED, EdgeOrientation::TURNED) => EdgeOrientation::FRONT,
            }
        }).collect::<Vec<EdgeOrientation>>().try_into().unwrap();
    }

    fn move_r(mut self) -> Self {
        self.apply_move(MOVE_R);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solved() {
        let cube = Cube::solved();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_move_r() {
        let cube = Cube::solved();
        let moved_cube = cube.move_r();

        assert_eq!(moved_cube.cp, [CornerPart::ULB, CornerPart::URB, CornerPart::URF, CornerPart::ULF, CornerPart::DLB, CornerPart::DRB, CornerPart::DRF, CornerPart::DLF]);
        assert_eq!(moved_cube.co, [CornerOrientation::FRONT; 8]);
        assert_eq!(moved_cube.ep, [EdgePart::LB, EdgePart::BR, EdgePart::RF, EdgePart::FL, EdgePart::UB, EdgePart::UR, EdgePart::UF, EdgePart::UL, EdgePart::DB, EdgePart::DR, EdgePart::DF, EdgePart::DL]);
        assert_eq!(moved_cube.eo, [EdgeOrientation::FRONT; 12]);
    }
}
