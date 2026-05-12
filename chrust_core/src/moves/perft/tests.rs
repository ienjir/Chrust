use crate::position::load_position_from_fen;

// ── helpers ──────────────────────────────────────────────────────────────────

fn perft(fen: &str, depth: u32) -> u64 {
	let mut pos = load_position_from_fen(fen).expect("invalid FEN");
	pos.perft(depth)
}

// ── Position 1 : starting position ──────────────────────────────────────────
// https://www.chessprogramming.org/Perft_Results#Initial_Position

const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn start_d1() {
	assert_eq!(perft(START, 1), 20);
}

#[test]
fn start_d2() {
	assert_eq!(perft(START, 2), 400);
}

#[test]
fn start_d3() {
	assert_eq!(perft(START, 3), 8902);
}

#[test]
#[ignore]
fn start_d4() {
	assert_eq!(perft(START, 4), 197_281);
}

#[test]
#[ignore]
fn start_d5() {
	assert_eq!(perft(START, 5), 4_865_609);
}

// ── Position 2 : Kiwipete ────────────────────────────────────────────────────
// https://www.chessprogramming.org/Perft_Results#Position_2

const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

#[test]
fn kiwipete_d1() {
	assert_eq!(perft(KIWIPETE, 1), 48);
}

#[test]
fn kiwipete_d2() {
	assert_eq!(perft(KIWIPETE, 2), 2039);
}

#[test]
fn kiwipete_d3() {
	assert_eq!(perft(KIWIPETE, 3), 97_862);
}

#[test]
#[ignore]
fn kiwipete_d4() {
	assert_eq!(perft(KIWIPETE, 4), 4_085_603);
}

// ── Position 3 ───────────────────────────────────────────────────────────────
// https://www.chessprogramming.org/Perft_Results#Position_3

const POS3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

#[test]
fn pos3_d1() {
	assert_eq!(perft(POS3, 1), 14);
}

#[test]
fn pos3_d2() {
	assert_eq!(perft(POS3, 2), 191);
}

#[test]
fn pos3_d3() {
	assert_eq!(perft(POS3, 3), 2812);
}

#[test]
#[ignore]
fn pos3_d4() {
	assert_eq!(perft(POS3, 4), 43_238);
}

#[test]
#[ignore]
fn pos3_d5() {
	assert_eq!(perft(POS3, 5), 674_624);
}

// ── Position 5 ───────────────────────────────────────────────────────────────
// https://www.chessprogramming.org/Perft_Results#Position_5

const POS5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

#[test]
fn pos5_d1() {
	assert_eq!(perft(POS5, 1), 44);
}

#[test]
fn pos5_d2() {
	assert_eq!(perft(POS5, 2), 1486);
}

#[test]
fn pos5_d3() {
	assert_eq!(perft(POS5, 3), 62_379);
}

#[test]
#[ignore]
fn pos5_d4() {
	assert_eq!(perft(POS5, 4), 2_103_487);
}

// ── Position 6 ───────────────────────────────────────────────────────────────
// https://www.chessprogramming.org/Perft_Results#Position_6

const POS6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

#[test]
fn pos6_d1() {
	assert_eq!(perft(POS6, 1), 46);
}

#[test]
fn pos6_d2() {
	assert_eq!(perft(POS6, 2), 2079);
}

#[test]
fn pos6_d3() {
	assert_eq!(perft(POS6, 3), 89_890);
}

#[test]
#[ignore]
fn pos6_d4() {
	assert_eq!(perft(POS6, 4), 3_894_594);
}
