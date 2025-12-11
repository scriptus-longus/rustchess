use criterion::{criterion_group, criterion_main, Criterion};
use rustchess::{movegen, game};

pub fn criterion_benchmark(c: &mut Criterion) {
    let game = match game::GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") {
      Ok(x) => x,
      Err(_) => {
        println!("could not create board");
        return ();
      },
    };

    c.bench_function("start position move generation", |b| {
      b.iter(|| {
       movegen::MoveGen::pseudo_legal(&game);  
      });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

