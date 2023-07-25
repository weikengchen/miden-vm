use criterion::{criterion_group, criterion_main, Criterion};
use miden::{prove, Assembler, MemAdviceProvider, ProofOptions, StackInputs};
use std::time::Duration;
use stdlib::StdLibrary;

fn program_prove(c: &mut Criterion) {
    let mut group = c.benchmark_group("program_prove");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("sha256", |bench| {
        let source = "
            use.std::crypto::hashes::sha256

            begin
                exec.sha256::hash_2to1
                exec.sha256::hash_2to1
                exec.sha256::hash_2to1
                exec.sha256::hash_2to1
                exec.sha256::hash_2to1
                exec.sha256::hash_2to1
                exec.sha256::hash_2to1
            end";
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .expect("failed to load stdlib");
        let program = assembler.compile(source).expect("Failed to compile test source.");
        bench.iter(|| {
            prove(
                &program,
                StackInputs::default(),
                MemAdviceProvider::default(),
                ProofOptions::default(),
            )
        });
    });

    group.finish();
}

criterion_group!(sha256_group, program_prove);
criterion_main!(sha256_group);
