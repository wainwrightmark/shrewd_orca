use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use shrewd_orca::{core::prelude::*, language::prelude::*};
use std::rc::Rc;

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(instant::Duration::new(5, 0));

    targets= bench_solver,


);
criterion_main!(benches);

fn bench_solver(c: &mut Criterion) {
    let context = Rc::new(WordContext::from_data());

    let mut group = c.benchmark_group("solver");
    group.sample_size(10);

    group.bench_function("Find a husband for Emma", |bench| {
        bench.iter(|| answer_question(context.clone(), "Emma #l =a !phrase"))
    });
    
    
    group.bench_function("Find a husband for Emma", |bench| {
        bench.iter(|| answer_question(context.clone(), "Emma #l =a !phrase"))
    });

    group.finish()
}

fn answer_question(context: Rc<WordContext>, input_str: &str) {
    let p = question_parse(input_str).unwrap();
    let solutions = p.solve(&context).take(10).collect_vec();

    assert!(!solutions.is_empty())
}
