#[macro_use]

extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use tokenz::Tokenizer;


fn bench_tokenizer(c: &mut Criterion) {
    let tokenizer = Tokenizer::english();
    let text = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
    c.bench_function("tokenize poem", move |b| b.iter(|| tokenizer.tokenize(black_box(text))));
}

criterion_group!(benches, bench_tokenizer);
criterion_main!(benches);
