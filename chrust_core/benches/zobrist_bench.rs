use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::load_position_from_fen;
use chrust_core::zobrist::{piece_index, zobrist};
use chrust_core::{ColoredPiece, Piece, Side};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_zobrist_table_access(c: &mut Criterion) {
	c.bench_function("zobrist_table_access", |b| {
		b.iter(|| {
			let z = zobrist();
			black_box(z);
		});
	});
}

fn bench_piece_index_calculation(c: &mut Criterion) {
	let piece = ColoredPiece { piece: Piece::Queen, side: Side::Black };

	c.bench_function("piece_index_calculation", |b| {
		b.iter(|| {
			let idx = piece_index(black_box(piece));
			black_box(idx);
		});
	});
}

fn bench_compute_hash_empty_board(c: &mut Criterion) {
	let pos = load_position_from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();

	c.bench_function("compute_hash_empty_board", |b| {
		b.iter(|| {
			let hash = black_box(&pos).compute_hash();
			black_box(hash);
		});
	});
}

fn bench_compute_hash_starting_position(c: &mut Criterion) {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

	c.bench_function("compute_hash_starting_position", |b| {
		b.iter(|| {
			let hash = black_box(&pos).compute_hash();
			black_box(hash);
		});
	});
}

fn bench_compute_hash_middlegame(c: &mut Criterion) {
	let pos = load_position_from_fen("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 1").unwrap();

	c.bench_function("compute_hash_middlegame", |b| {
		b.iter(|| {
			let hash = black_box(&pos).compute_hash();
			black_box(hash);
		});
	});
}

fn bench_incremental_hash_quiet_move(c: &mut Criterion) {
	let mut pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 1,
		to_square: 18,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};

	c.bench_function("incremental_hash_quiet_move", |b| {
		b.iter(|| {
			let mut test_pos = pos.clone();
			test_pos.make_move_unvalidated(black_box(mv)).unwrap();
			black_box(test_pos.zobrist_hash);
		});
	});
}

fn bench_incremental_hash_capture(c: &mut Criterion) {
	let mut pos = load_position_from_fen("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 19,
		to_square: 28,
		move_kind: MoveKind::Capture,
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	c.bench_function("incremental_hash_capture", |b| {
		b.iter(|| {
			let mut test_pos = pos.clone();
			test_pos.make_move_unvalidated(black_box(mv)).unwrap();
			black_box(test_pos.zobrist_hash);
		});
	});
}

fn bench_incremental_hash_castling(c: &mut Criterion) {
	let mut pos = load_position_from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 4,
		to_square: 6,
		move_kind: MoveKind::Castling { rook_from: 7, rook_to: 5 },
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
	};

	c.bench_function("incremental_hash_castling", |b| {
		b.iter(|| {
			let mut test_pos = pos.clone();
			test_pos.make_move_unvalidated(black_box(mv)).unwrap();
			black_box(test_pos.zobrist_hash);
		});
	});
}

fn bench_hash_update_vs_recompute(c: &mut Criterion) {
	let mut group = c.benchmark_group("hash_update_vs_recompute");

	let mut pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 1,
		to_square: 18,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};

	group.bench_function("incremental_update", |b| {
		b.iter(|| {
			let mut test_pos = pos.clone();
			test_pos.make_move_unvalidated(black_box(mv)).unwrap();
			black_box(test_pos.zobrist_hash);
		});
	});

	group.bench_function("full_recompute", |b| {
		b.iter(|| {
			let mut test_pos = pos.clone();
			test_pos.make_move_unvalidated(black_box(mv)).unwrap();
			let hash = test_pos.compute_hash();
			black_box(hash);
		});
	});

	group.finish();
}

criterion_group!(
	benches,
	bench_zobrist_table_access,
	bench_piece_index_calculation,
	bench_compute_hash_empty_board,
	bench_compute_hash_starting_position,
	bench_compute_hash_middlegame,
	bench_incremental_hash_quiet_move,
	bench_incremental_hash_capture,
	bench_incremental_hash_castling,
	bench_hash_update_vs_recompute
);

criterion_main!(benches);
