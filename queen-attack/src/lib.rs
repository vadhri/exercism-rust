#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen {
    cp: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let valid = ( rank >= 0 && rank < 8 ) && ( file >= 0 && file < 8 );

        match valid {
            true => Some(ChessPosition {
                rank: rank,
                file: file
            }),
            false => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            cp: position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let horizontal_attack = self.cp.rank == other.cp.rank;
        let vertical_attack = self.cp.file == other.cp.file;
        let diagonal_attack = (self.cp.rank - other.cp.rank).abs() == (self.cp.file - other.cp.file).abs();

        horizontal_attack || vertical_attack || diagonal_attack
    }
}
