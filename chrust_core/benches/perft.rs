use chrust_core::position::load_position_from_fen;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_perft(c: &mut Criterion, label: &str, fen: &str, depths: &[u32]) {
	let mut group = c.benchmark_group(label);
	for &depth in depths {
		group.bench_with_input(BenchmarkId::new("perft", depth), &depth, |b, &d| {
			b.iter(|| {
				let mut pos = load_position_from_fen(fen).unwrap();
				pos.perft(d)
			});
		});
	}
	group.finish();
}

fn benchmark_starting_position(c: &mut Criterion) {
	bench_perft(c, "starting_position", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &[3, 4, 5]);
}

fn benchmark_kiwipete(c: &mut Criterion) {
	bench_perft(c, "kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &[3, 4]);
}

fn benchmark_pos3(c: &mut Criterion) {
	bench_perft(c, "position_3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &[4, 5]);
}

fn benchmark_pos5(c: &mut Criterion) {
	bench_perft(c, "position_5", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &[3, 4]);
}

criterion_group!(benches, benchmark_starting_position, benchmark_kiwipete, benchmark_pos3, benchmark_pos5,);
criterion_main!(benches);
