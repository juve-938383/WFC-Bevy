pub mod config {

    use std::collections::HashMap;

    //Map size
    pub const MAP_WIDTH: i32 = 40;
    pub const MAP_HEIGHT: i32 = 21;

    //Directions
    pub const NORTH: usize = 0;
    pub const EAST: usize = 1;
    pub const SOUTH: usize = 2;
    pub const WEST: usize = 3;

    pub const DIRECTIONS: [usize; 4] = [
        NORTH,
        EAST,
        SOUTH,
        WEST
    ];
    pub const OPPOSITE_DIRECTIONS: [usize; 4] = [
        SOUTH,
        WEST,
        NORTH,
        EAST
    ];

    //Tile Types
    pub const G: usize = 0;
    pub const G2: usize = 1;
    pub const G_C_E: usize = 2;
    pub const G_C_N: usize = 3;
    pub const G_C_NE_C: usize = 4;
    pub const G_C_NE_O: usize = 5;
    pub const G_C_NW_C: usize = 6;
    pub const G_C_NW_O: usize = 7;
    pub const G_C_S: usize = 8;
    pub const G_C_SE_C: usize = 9;
    pub const G_C_SE_O: usize = 10;
    pub const G_C_SW_C: usize = 11;
    pub const G_C_SW_O: usize = 12;
    pub const G_C_W: usize = 13;
    pub const G_I_L: usize = 14;
    pub const G_I_S: usize = 15;
    pub const G_P_CABIN: usize = 16;
    pub const G_P_FLAG: usize = 17;
    pub const G_P_HOUSE: usize = 18;
    pub const G_P_LAMP: usize = 19;
    pub const G_P_PLANT: usize = 20;
    pub const G_P_TREE: usize = 21;
    pub const G_P_TREES: usize = 22;
    pub const G_S_E: usize = 23;
    pub const G_S_ES: usize = 24;
    pub const G_S_ESW: usize = 25;
    pub const G_S_EW: usize = 26;
    pub const G_S_EW2: usize = 27;
    pub const G_S_N: usize = 28;
    pub const G_S_NE: usize = 29;
    pub const G_S_NES: usize = 30;
    pub const G_S_NESW: usize = 31;
    pub const G_S_NEW: usize = 32;
    pub const G_S_NS: usize = 33;
    pub const G_S_NS2: usize = 34;
    pub const G_S_NW: usize = 35;
    pub const G_S_NSW: usize = 36;
    pub const G_S_S: usize = 37;
    pub const G_S_SW: usize = 38;
    pub const G_S_W: usize = 39;
    pub const S: usize = 40;
    pub const S2: usize = 41;
    pub const S_C_E: usize = 42;
    pub const S_C_N: usize = 43;
    pub const S_C_NE_C: usize = 44;
    pub const S_C_NE_O: usize = 45;
    pub const S_C_NW_C: usize = 46;
    pub const S_C_NW_O: usize = 47;
    pub const S_C_S: usize = 48;
    pub const S_C_SE_C: usize = 49;
    pub const S_C_SE_O: usize = 50;
    pub const S_C_SW_C: usize = 51;
    pub const S_C_SW_O: usize = 52;
    pub const S_C_W: usize = 53;
    pub const S_I_L: usize = 54;
    pub const S_I_S: usize = 55;
    pub const S_P_CABIN: usize = 56;
    pub const S_P_FLAG: usize = 57;
    pub const S_P_HOUSE: usize = 58;
    pub const S_P_LAMP: usize = 59;
    pub const S_P_TREE: usize = 60;
    pub const S_P_TREES: usize = 61;
    pub const S_S_E: usize = 62;
    pub const S_S_ES: usize = 63;
    pub const S_S_ESW: usize = 64;
    pub const S_S_EW: usize = 65;
    pub const S_S_EW2: usize = 66;
    pub const S_S_N: usize = 67;
    pub const S_S_NE: usize = 68;
    pub const S_S_NES: usize = 69;
    pub const S_S_NESW: usize = 70;
    pub const S_S_NEW: usize = 71;
    pub const S_S_NS: usize = 72;
    pub const S_S_NS2: usize = 73;
    pub const S_S_NSW: usize = 74;
    pub const S_S_NW: usize = 75;
    pub const S_S_S: usize = 76;
    pub const S_S_SW: usize = 77;
    pub const S_S_W: usize = 78;
    pub const W: usize = 79;
    pub const DEFAULT_DARK: usize = 80;
    pub const DEFAULT_LIGHT: usize = 81;

    //Tile Edges
    pub const GRASS: usize = 0;
    pub const WATER: usize = 1;
    pub const COAST_G_N: usize = 2;
    pub const COAST_G_E: usize = 3;
    pub const COAST_G_S: usize = 4;
    pub const COAST_G_W: usize = 5;
    pub const COAST_S_N: usize = 6;
    pub const COAST_S_E: usize = 7;
    pub const COAST_S_S: usize = 8;
    pub const COAST_S_W: usize = 9;
    pub const SOIL: usize = 10;
    pub const STREET_G_H: usize = 11;
    pub const STREET_G_V: usize = 12;
    pub const STREET_S_H: usize = 13;
    pub const STREET_S_V: usize = 14;


    //Tile Rules
    pub const TILE_RULES: [[usize;4]; 80] = [
    [GRASS, GRASS, GRASS, GRASS], //G
    [GRASS, GRASS, GRASS, GRASS], //G2
    [COAST_G_E, WATER, COAST_G_E, GRASS], //G_C_E
    [WATER, COAST_G_N, GRASS, COAST_G_N], //G_C_N
    [COAST_G_E, COAST_G_N, GRASS, GRASS], //G_C_NE_C
    [WATER, WATER, COAST_G_E, COAST_G_N], //G_C_NE_O
    [COAST_G_W, GRASS, GRASS, COAST_G_N], //G_C_NW_C
    [WATER, COAST_G_N, COAST_G_W, WATER], //G_C_NW_O
    [GRASS, COAST_G_S, WATER, COAST_G_S], //G_C_S
    [GRASS, COAST_G_S, COAST_G_E, GRASS], //G_C_SE_C
    [COAST_G_E, WATER, WATER, COAST_G_S], //G_C_SE_O
    [GRASS, GRASS, COAST_G_W, COAST_G_S], //G_C_SW_C
    [COAST_G_W, COAST_G_S, WATER, WATER], //G_C_SW_O
    [COAST_G_W, GRASS, COAST_G_W, WATER], //G_C_W
    [WATER, WATER, WATER, WATER], //G_I_L
    [WATER, WATER, WATER, WATER], //G_I_S
    [GRASS, GRASS, GRASS, GRASS], //G_P_CABIN
    [GRASS, GRASS, GRASS, GRASS], //G_P_FLAG
    [GRASS, GRASS, GRASS, GRASS], //G_P_HOUSE
    [GRASS, GRASS, GRASS, GRASS], //G_P_LAMP
    [GRASS, GRASS, GRASS, GRASS], //G_P_PLANT
    [GRASS, GRASS, GRASS, GRASS], //G_P_TREE
    [GRASS, GRASS, GRASS, GRASS], //G_P_TREES
    [GRASS, STREET_G_H, GRASS, GRASS], //G_S_E
    [GRASS, STREET_G_H, STREET_G_V, GRASS], //G_S_ES
    [GRASS, STREET_G_H, STREET_G_V, STREET_G_H], //G_S_ESW
    [GRASS, STREET_G_H, GRASS, STREET_G_H], //G_S_EW
    [GRASS, STREET_G_H, GRASS, STREET_G_H], //G_S_EW2
    [STREET_G_V, GRASS, GRASS, GRASS], //G_S_N
    [STREET_G_V, STREET_G_H, GRASS, GRASS], //G_S_NE
    [STREET_G_V, STREET_G_H, STREET_G_V, GRASS], //G_S_NES
    [STREET_G_V, STREET_G_H, STREET_G_V, STREET_G_H], //G_S_NESW
    [STREET_G_V, STREET_G_H, GRASS, STREET_G_H], //G_S_NEW
    [STREET_G_V, GRASS, STREET_G_V, GRASS], //G_S_NS
    [STREET_G_V, GRASS, STREET_G_V, GRASS], //G_S_NS2
    [STREET_G_V, GRASS, GRASS, STREET_G_H], //G_S_NW
    [STREET_G_V, GRASS, STREET_G_V, STREET_G_H], //G_S_NSW
    [GRASS, GRASS, STREET_G_V, GRASS], //G_S_S
    [GRASS, GRASS, STREET_G_V, STREET_G_H], //G_S_SW
    [GRASS, GRASS, GRASS, STREET_G_H], //G_S_W
    [SOIL, SOIL, SOIL, SOIL], //S
    [SOIL, SOIL, SOIL, SOIL], //S2
    [COAST_S_E, WATER, COAST_S_E, SOIL], //S_C_E
    [WATER, COAST_S_N, SOIL, COAST_S_N], //S_C_N
    [COAST_S_E, COAST_S_N, SOIL, SOIL], //S_C_NE_C
    [WATER, WATER, COAST_S_E, COAST_S_N], //S_C_NE_O
    [COAST_S_W, SOIL, SOIL, COAST_S_N], //S_C_NW_C
    [WATER, COAST_S_N, COAST_S_W, WATER], //S_C_NW_O
    [SOIL, COAST_S_S, WATER, COAST_S_S], //S_C_S
    [SOIL, COAST_S_S, COAST_S_E, SOIL], //S_C_SE_C
    [COAST_S_E, WATER, WATER, COAST_S_S], //S_C_SE_O
    [SOIL, SOIL, COAST_S_W, COAST_S_S], //S_C_SW_C
    [COAST_S_W, COAST_S_S, WATER, WATER], //S_C_SW_O
    [COAST_S_W, SOIL, COAST_S_W, WATER], //S_C_W
    [WATER, WATER, WATER, WATER], //S_I_L
    [WATER, WATER, WATER, WATER], //S_I_S
    [SOIL, SOIL, SOIL, SOIL], //S_P_CABIN
    [SOIL, SOIL, SOIL, SOIL], //S_P_FLAG
    [SOIL, SOIL, SOIL, SOIL], //S_P_HOUSE
    [SOIL, SOIL, SOIL, SOIL], //S_P_LAMP
    [SOIL, SOIL, SOIL, SOIL], //S_P_TREE
    [SOIL, SOIL, SOIL, SOIL], //S_P_TREES
    [SOIL, STREET_S_H, SOIL, SOIL], //S_S_E
    [SOIL, STREET_S_H, STREET_S_V, SOIL], //S_S_ES
    [SOIL, STREET_S_H, STREET_S_V, STREET_S_H], //S_S_ESW
    [SOIL, STREET_S_H, SOIL, STREET_S_H], //S_S_EW
    [SOIL, STREET_S_H, SOIL, STREET_S_H], //S_S_EW2
    [STREET_S_V, SOIL, SOIL, SOIL], //S_S_N
    [STREET_S_V, STREET_S_H, SOIL, SOIL], //S_S_NE
    [STREET_S_V, STREET_S_H, STREET_S_V, SOIL], //S_S_NES
    [STREET_S_V, STREET_S_H, STREET_S_V, STREET_S_H], //S_S_NESW
    [STREET_S_V, STREET_S_H, SOIL, STREET_S_H], //S_S_NEW
    [STREET_S_V, SOIL, STREET_S_V, SOIL], //S_S_NS
    [STREET_S_V, SOIL, STREET_S_V, SOIL], //S_S_NS2
    [STREET_S_V, SOIL, STREET_S_V, STREET_S_H], //S_S_NSW
    [STREET_S_V, SOIL, SOIL, STREET_S_H], //S_S_NW
    [SOIL, SOIL, STREET_S_V, SOIL], //S_S_S
    [SOIL, SOIL, STREET_S_V, STREET_S_H], //S_S_SW
    [SOIL, SOIL, SOIL, STREET_S_H], //S_S_W
    [WATER, WATER, WATER, WATER] //W
    ];


    //Tile Weights
    pub const TILE_WEIGHTS: [usize; 80] = [
        300, //G
        300, //G2
        60, //G_C_E
        60, //G_C_N
        10, //G_C_NE_C
        10, //G_C_NE_O
        10, //G_C_NW_C
        10, //G_C_NW_O
        60, //G_C_S
        10, //G_C_SE_C
        10, //G_C_SE_O
        10, //G_C_SW_C
        10, //G_C_SW_O
        60, //G_C_W
        10, //G_I_L
        1, //G_I_S
        20, //G_P_CABIN
        6, //G_P_FLAG
        20, //G_P_HOUSE
        10, //G_P_LAMP
        20, //G_P_PLANT
        100, //G_P_TREE
        140, //G_P_TREES
        10, //G_S_E
        5, //G_S_ES
        1, //G_S_ESW
        20, //G_S_EW
        20, //G_S_EW2
        5, //G_S_N
        5, //G_S_NE
        1, //G_S_NES
        1, //G_S_NESW
        1, //G_S_NEW
        20, //G_S_NS
        20, //G_S_NS2
        5, //G_S_NW
        1, //G_S_NSW
        5, //G_S_S
        5, //G_S_SW
        5, //G_S_W
        300, //S
        300, //S2
        60, //S_C_E
        60, //S_C_N
        10, //S_C_NE_C
        10, //S_C_NE_O
        10, //S_C_NW_C
        10, //S_C_NW_O
        60, //S_C_S
        10, //S_C_SE_C
        10, //S_C_SE_O
        10, //S_C_SW_C
        10, //S_C_SW_O
        60, //S_C_W
        10, //S_I_L
        1, //S_I_S
        20, //S_P_CABIN
        6, //S_P_FLAG
        20, //S_P_HOUSE
        10, //S_P_LAMP
        100, //S_P_TREE
        140, //S_P_TREES
        5, //S_S_E
        5, //S_S_ES
        1, //S_S_ESW
        20, //S_S_EW
        20, //S_S_EW2
        5, //S_S_N
        5, //S_S_NE
        1, //S_S_NES
        1, //S_S_NESW
        1, //S_S_NEW
        20, //S_S_NS
        20, //S_S_NS2
        1, //S_S_NSW
        5, //S_S_NW
        5, //S_S_S
        3, //S_S_SW
        5, //S_S_W
        500, //W
    ];

    //Tile Sprite Paths
    pub const TILE_SPRITES: [&str; 80] = [
        "../assets/sprites/tiles/g.png",
        "../assets/sprites/tiles/g2.png",
        "../assets/sprites/tiles/g_c_e.png",
        "../assets/sprites/tiles/g_c_n.png",
        "../assets/sprites/tiles/g_c_ne_c.png",
        "../assets/sprites/tiles/g_c_ne_o.png",
        "../assets/sprites/tiles/g_c_nw_c.png",
        "../assets/sprites/tiles/g_c_nw_o.png",
        "../assets/sprites/tiles/g_c_s.png",
        "../assets/sprites/tiles/g_c_se_c.png",
        "../assets/sprites/tiles/g_c_se_o.png",
        "../assets/sprites/tiles/g_c_sw_c.png",
        "../assets/sprites/tiles/g_c_sw_o.png",
        "../assets/sprites/tiles/g_c_w.png",
        "../assets/sprites/tiles/g_i_l.png",
        "../assets/sprites/tiles/g_i_s.png",
        "../assets/sprites/tiles/g_p_cabin.png",
        "../assets/sprites/tiles/g_p_flag.png",
        "../assets/sprites/tiles/g_p_house.png",
        "../assets/sprites/tiles/g_p_lamp.png",
        "../assets/sprites/tiles/g_p_plant.png",
        "../assets/sprites/tiles/g_p_tree.png",
        "../assets/sprites/tiles/g_p_trees.png",
        "../assets/sprites/tiles/g_s_e.png",
        "../assets/sprites/tiles/g_s_es.png",
        "../assets/sprites/tiles/g_s_esw.png",
        "../assets/sprites/tiles/g_s_ew.png",
        "../assets/sprites/tiles/g_s_ew2.png",
        "../assets/sprites/tiles/g_s_n.png",
        "../assets/sprites/tiles/g_s_ne.png",
        "../assets/sprites/tiles/g_s_nes.png",
        "../assets/sprites/tiles/g_s_nesw.png",
        "../assets/sprites/tiles/g_s_new.png",
        "../assets/sprites/tiles/g_s_ns.png",
        "../assets/sprites/tiles/g_s_ns2.png",
        "../assets/sprites/tiles/g_s_nw.png",
        "../assets/sprites/tiles/g_s_nsw.png",
        "../assets/sprites/tiles/g_s_s.png",
        "../assets/sprites/tiles/g_s_sw.png",
        "../assets/sprites/tiles/g_s_w.png",
        "../assets/sprites/tiles/s.png",
        "../assets/sprites/tiles/s2.png",
        "../assets/sprites/tiles/s_c_e.png",
        "../assets/sprites/tiles/s_c_n.png",
        "../assets/sprites/tiles/s_c_ne_c.png",
        "../assets/sprites/tiles/s_c_ne_o.png",
        "../assets/sprites/tiles/s_c_nw_c.png",
        "../assets/sprites/tiles/s_c_nw_o.png",
        "../assets/sprites/tiles/s_c_s.png",
        "../assets/sprites/tiles/s_c_se_c.png",
        "../assets/sprites/tiles/s_c_se_o.png",
        "../assets/sprites/tiles/s_c_sw_c.png",
        "../assets/sprites/tiles/s_c_sw_o.png",
        "../assets/sprites/tiles/s_c_w.png",
        "../assets/sprites/tiles/s_i_l.png",
        "../assets/sprites/tiles/s_i_s.png",
        "../assets/sprites/tiles/s_p_cabin.png",
        "../assets/sprites/tiles/s_p_flag.png",
        "../assets/sprites/tiles/s_p_house.png",
        "../assets/sprites/tiles/s_p_lamp.png",
        "../assets/sprites/tiles/s_p_tree.png",
        "../assets/sprites/tiles/s_p_trees.png",
        "../assets/sprites/tiles/s_s_e.png",
        "../assets/sprites/tiles/s_s_es.png",
        "../assets/sprites/tiles/s_s_esw.png",
        "../assets/sprites/tiles/s_s_ew.png",
        "../assets/sprites/tiles/s_s_ew2.png",
        "../assets/sprites/tiles/s_s_n.png",
        "../assets/sprites/tiles/s_s_ne.png",
        "../assets/sprites/tiles/s_s_nes.png",
        "../assets/sprites/tiles/s_s_nesw.png",
        "../assets/sprites/tiles/s_s_new.png",
        "../assets/sprites/tiles/s_s_ns.png",
        "../assets/sprites/tiles/s_s_ns2.png",
        "../assets/sprites/tiles/s_s_nsw.png",
        "../assets/sprites/tiles/s_s_nw.png",
        "../assets/sprites/tiles/s_s_s.png",
        "../assets/sprites/tiles/s_s_sw.png",
        "../assets/sprites/tiles/s_s_w.png",
        "../assets/sprites/tiles/w.png"
    ];

    pub const ALL_TILES: [usize; 80] = [
    G, G2, G_C_E, G_C_N, G_C_NE_C, G_C_NE_O, G_C_NW_C, G_C_NW_O, G_C_S, G_C_SE_C,
    G_C_SE_O, G_C_SW_C, G_C_SW_O, G_C_W, G_I_L, G_I_S, G_P_CABIN, G_P_FLAG, G_P_HOUSE, G_P_LAMP,
    G_P_PLANT, G_P_TREE, G_P_TREES, G_S_E, G_S_ES, G_S_ESW, G_S_EW, G_S_EW2, G_S_N, G_S_NE,
    G_S_NES, G_S_NESW, G_S_NEW, G_S_NS, G_S_NS2, G_S_NW, G_S_NSW, G_S_S, G_S_SW, G_S_W,
    S, S2, S_C_E, S_C_N, S_C_NE_C, S_C_NE_O, S_C_NW_C, S_C_NW_O, S_C_S, S_C_SE_C,
    S_C_SE_O, S_C_SW_C, S_C_SW_O, S_C_W, S_I_L, S_I_S, S_P_CABIN, S_P_FLAG, S_P_HOUSE, S_P_LAMP,
    S_P_TREE, S_P_TREES, S_S_E, S_S_ES, S_S_ESW, S_S_EW, S_S_EW2, S_S_N, S_S_NE, S_S_NES,
    S_S_NESW, S_S_NEW, S_S_NS, S_S_NS2, S_S_NSW, S_S_NW, S_S_S, S_S_SW, S_S_W, W
    ];

}