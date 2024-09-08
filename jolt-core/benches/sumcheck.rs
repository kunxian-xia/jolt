use ark_bn254::Fr;
use ark_std::rand::prelude::ThreadRng;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use jolt_core::poly::dense_mlpoly::DensePolynomial;
use jolt_core::subprotocols::sumcheck::SumcheckInstanceProof;
use jolt_core::utils::transcript::ProofTranscript;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

fn bench_sumcheck_internal(c: &mut Criterion, num_prods: usize) {
    let mut group = c.benchmark_group(format!("sumcheck<prod={}>", num_prods));
    group.sample_size(10);

    let mut rng = ThreadRng::default();
    for nv in 20..24 {
        if num_prods == 2 {
            group.bench_function(BenchmarkId::new("prove_quadratic", nv), |b| {
                b.iter_batched(
                    || {
                        let poly_a = DensePolynomial::<Fr>::random(nv, &mut rng);
                        let poly_b = DensePolynomial::<Fr>::random(nv, &mut rng);
                        let transcript = ProofTranscript::new(b"bench_sumcheck");

                        let expected_sum = poly_a
                            .Z
                            .par_iter()
                            .zip(poly_b.Z.par_iter())
                            .map(|(a, b)| a * b)
                            .sum::<Fr>();

                        (poly_a, poly_b, expected_sum, transcript)
                    },
                    |(mut poly_a, mut poly_b, expected_sum, mut transcript)| {
                        SumcheckInstanceProof::<Fr>::prove_quadratic(
                            &expected_sum,
                            nv,
                            &mut poly_a,
                            &mut poly_b,
                            &mut transcript,
                        );
                    },
                    BatchSize::SmallInput,
                );
            });
        } else if num_prods == 3 {
        }
    }
}

fn bench_sumcheck(c: &mut Criterion) {
    // bench sumcheck prover for product of two multilinear polynomials
    bench_sumcheck_internal(c, 2);

    // TODO: bench sumcheck for product of three multilinear polynomials
}
criterion_group!(benches, bench_sumcheck);
criterion_main!(benches);
