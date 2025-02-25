use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

use toroid::Donut;

const WIDTH: u8 = 80;
const HEIGHT: u8 = 22;
const SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

type BenchDonut = Donut<WIDTH, HEIGHT>;

fn bench_rotate_parameterized(c: &mut Criterion) {
    let mut donut = BenchDonut::new();
    let mut group = c.benchmark_group("rotate_angles");

    for &da in &[0.01, 0.1, 1.0] {
        for &db in &[0.01, 0.1, 1.0] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("Angle_A={:.2}, Angle_B={:.2}", da, db)),
                &(da, db),
                |b, &(da, db)| {
                    b.iter(|| {
                        donut.rotate(black_box(da), black_box(db));
                    });
                },
            );
        }
    }

    group.finish();
}

fn bench_render_parameterized(c: &mut Criterion) {
    let donut = BenchDonut::new();
    let mut group = c.benchmark_group("render_frames");

    for &frames in &[1, 10, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(frames),
            &frames,
            |b, &frames| {
                let mut output = vec![' '; SIZE];
                let mut zbuf = vec![0.0_f32; SIZE];
                b.iter(|| {
                    for _ in 0..frames {
                        donut.render_frame_in_place(black_box(&mut output), black_box(&mut zbuf));
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_rotate_and_render_parameterized(c: &mut Criterion) {
    let mut donut = BenchDonut::new();
    let mut group = c.benchmark_group("rotate_and_render_frames");

    for &frames in &[1, 10, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(frames),
            &frames,
            |b, &frames| {
                let mut output = vec![' '; SIZE];
                let mut zbuf = vec![0.0_f32; SIZE];
                b.iter(|| {
                    for _ in 0..frames {
                        donut.rotate(black_box(0.01), black_box(0.01));
                        donut.render_frame_in_place(black_box(&mut output), black_box(&mut zbuf));
                    }
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_rotate_parameterized,
    bench_render_parameterized,
    bench_rotate_and_render_parameterized
);
criterion_main!(benches);
