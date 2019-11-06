use criterion::{self, criterion_group, criterion_main, Criterion};
use rand::RngCore;
use rustc_hex::{FromHex, ToHex};

fn encode(criterion: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut group = criterion.benchmark_group("encode");
    for size in &[32usize, 1024, 65536] {
        let mut data = vec![0u8; *size];
        rng.fill_bytes(&mut data);

        group.bench_with_input(format!("hex [{} bytes]", size), &data, |bencher, data| {
            bencher.iter(|| criterion::black_box(hex::encode(data)))
        });

        group.bench_with_input(
            format!("rustc_hex [{} bytes]", size),
            &data,
            |bencher, data| bencher.iter(|| criterion::black_box(data.to_hex::<String>())),
        );

        group.bench_with_input(
            format!("faster_hex [{} bytes]", size),
            &data,
            |bencher, data| {
                bencher.iter(|| criterion::black_box(faster_hex::hex_string(data).unwrap()))
            },
        );

        group.bench_with_input(
            format!("base16 [{} bytes]", size),
            &data,
            |bencher, data| bencher.iter(|| criterion::black_box(base16::encode_lower(data))),
        );

        group.bench_with_input(
            format!("binascii [{} bytes]", size),
            &data,
            |bencher, data| {
                bencher.iter(|| {
                    let mut output = vec![0; size * 2];
                    binascii::bin2hex(data, &mut output).unwrap();
                    criterion::black_box(output)
                })
            },
        );

        group.bench_with_input(
            format!("casperlabs [{} bytes]", size),
            &data,
            |bencher, data| {
                bencher.iter(|| {
                    criterion::black_box(casperlabs_contract_ffi::base16::encode_lower(data))
                })
            },
        );
    }
}

fn decode(criterion: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut group = criterion.benchmark_group("decode");

    for size in &[32usize, 1024, 65536] {
        let mut data = vec![0u8; *size];
        rng.fill_bytes(&mut data);
        let hex_data = faster_hex::hex_string(&data).unwrap();

        group.bench_with_input(
            format!("hex [{} bytes]", size),
            &hex_data,
            |bencher, hex_data| {
                bencher.iter(|| criterion::black_box(hex::decode(hex_data).unwrap()))
            },
        );

        group.bench_with_input(
            format!("rustc_hex [{} bytes]", size),
            &hex_data,
            |bencher, hex_data| {
                bencher.iter(|| criterion::black_box(hex_data.from_hex::<Vec<u8>>().unwrap()))
            },
        );

        group.bench_with_input(
            format!("faster_hex [{} bytes]", size),
            &hex_data,
            |bencher, hex_data| {
                bencher.iter(|| {
                    let mut output = vec![0u8; *size];
                    faster_hex::hex_decode(hex_data.as_bytes(), &mut output).unwrap();
                    criterion::black_box(output)
                })
            },
        );

        group.bench_with_input(
            format!("base16 [{} bytes]", size),
            &hex_data,
            |bencher, hex_data| bencher.iter(|| criterion::black_box(base16::decode(hex_data).unwrap())),
        );

        group.bench_with_input(
            format!("binascii [{} bytes]", size),
            &hex_data,
            |bencher, hex_data| {
                bencher.iter(|| {
                    let mut output = vec![0u8; *size];
                    binascii::hex2bin(hex_data.as_bytes(), &mut output).unwrap();
                    criterion::black_box(output)
                })
            },
        );

        group.bench_with_input(
            format!("casperlabs [{} bytes]", size),
            &hex_data,
            |bencher, hex_data| {
                bencher.iter(|| {
                    criterion::black_box(
                        casperlabs_contract_ffi::base16::decode_lower(hex_data).unwrap(),
                    )
                })
            },
        );
    }
}

criterion_group!(benches, encode, decode);
criterion_main!(benches);
