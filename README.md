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
