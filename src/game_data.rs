#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct GameData {
    current_player: Player,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Player {
    White,
    Black
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct BoardSize {
    pub x: u8,
    pub y: u8,
}
