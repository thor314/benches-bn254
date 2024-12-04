<h1 align="center">
  benches-bn254
</h1>

<div align="center">
  <a href="https://x.com/cryptograthor">
    <img src="https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink" />
    <!-- ![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink) -->
  </a>
  </div>


## bn256 benchmarks
Benchmarks operations in the BN256 curve using the halo2curves library:
* G1 point addition
* G1 scalar multiplication
* Fr scalar field multiplication
* BN256 pairing
* native `u64` multiplication as a point of reference

```sh
cargo bench
# or
just bench
```

## results on Thor's laptop
`cat /proc/cpuinfo` snippet:

model name      : 13th Gen Intel(R) Core(TM) i7-1355U

cpu MHz         : 2650

```
Benchmarking arkworks BN254/G1 point addition: Collecting 100 samplarkworks BN254/G1 point addition
                        time:   [271.25 ns 280.23 ns 291.88 ns]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
Benchmarking arkworks BN254/G1 scalar multiplication: Warming up fBenchmarking arkworks BN254/G1 scalar multiplication: Collecting 100 arkworks BN254/G1 scalar multiplication
                        time:   [69.389 µs 71.208 µs 72.951 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
arkworks BN254/Pairing  time:   [789.47 µs 810.56 µs 828.80 µs]
arkworks BN254/Fr scalar field multiplication
                        time:   [16.139 ns 16.687 ns 17.279 ns]
Found 19 outliers among 100 measurements (19.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  16 (16.00%) high severe

     Running benches/h2c.rs (target/release/deps/h2c-76e58bee600996f2)
Gnuplot not found, using plotters backend
halo2curves BN256/G1 point addition
                        time:   [296.33 ns 297.93 ns 299.74 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
halo2curves BN256/G1 scalar multiplication
                        time:   [152.27 µs 158.70 µs 165.44 µs]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe
halo2curves BN256/Pairing
                        time:   [631.84 µs 638.22 µs 645.44 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
halo2curves BN256/Fr scalar field multiplication
                        time:   [17.088 ns 17.124 ns 17.159 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
halo2curves BN256/native mul
                        time:   [430.98 ps 432.01 ps 433.10 ps]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high severe
````
