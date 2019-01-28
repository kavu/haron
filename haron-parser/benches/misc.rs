use criterion::*;

use haron_parser::misc::Core;

fn bench_core_iterator_next(c: &mut Criterion) {
    let mut core: Core<u64> = Core::with_capacity(10);

    core[0] = Some(0);
    core[1] = Some(1);
    core[3] = Some(3);

    c.bench_function("core.next()", move |b| b.iter(|| {
        core.next()
    }));
}

criterion_group!(core, bench_core_iterator_next);
