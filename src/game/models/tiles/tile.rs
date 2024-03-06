struct Tile {
    is_crumbling: bool,
    has_key: bool,
    orientation: Orientations,
}

enum Orientations {
    N = 0b1000,
    E = 0b0100,
    S = 0b0010,
    W = 0b0001,
}
