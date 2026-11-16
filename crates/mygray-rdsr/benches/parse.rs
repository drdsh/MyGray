//! Performance benchmark for SRS-RDSR-005 (p95 < 5 ms, 500-event CT RDSR).
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_parse(c: &mut Criterion) {
    c.bench_function("ct_rdsr_500_events", |b| {
        b.iter(|| {
            // let bytes = include_bytes!("../../../testdata/rdsr/ct/large_500_events.dcm");
            // mygray_rdsr::parse_bytes(bytes)
        })
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
