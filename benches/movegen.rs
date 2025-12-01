use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustchess::{movegen, board, Player};

pub fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let board = match board::Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") {
      Ok(x) => x,
      Err(_) => {
        println!("could not create board");
        return ();
      },
    };

    c.bench_function("start position move generation", |b| {
      b.iter(|| {
       movegen::MoveGen::pseudo_legal(&board, Player::White);  
      });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

