#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_0 = self.position.rank;
        let file_0 = self.position.file;
        let rank_1 = other.position.rank;
        let file_1 = other.position.file;
        rank_0 == rank_1 || file_0 == file_1 || (rank_0 - rank_1).abs() == (file_0 - file_1).abs()
    }
}
