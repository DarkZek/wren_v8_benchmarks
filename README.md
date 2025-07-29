## Rusty-v8 vs Wren benchmark

For the purpose of using as a game scripting language this conducts performance tests between Rusty-v8 (using the V8 Javascript engine) and Rewren (using wren).

[View the JavaScript benchmark implementation](./benches/script.js)

[View the Wren benchmark implementation](./benches/script.wren)

The benchmark is an iteration test, it takes two parameters A and B. While these numbers are low we should expect most of the time taken is interop & script parsing time. As these numbers increase the recursion increases so we should expect to measure more of the runtime performance of the engines.

On a M2 Pro Macbook Pro the results were as following

| Parameters (A, B) | Rewren | Rusty-V8 (No JIT) | Rusty-V8 (JIT) |
|-------------------|-------------|---------------------|-----------------------|
|1, 1|89ns|123.67ns|72.029ns|
|300, 50|2477.4ns|2445.5ns|504.06ns|
|3000, 500|24490ns|18552ns|4303.4ns|

### How to run

The test suite must be ran twice as v8 does not support changing flags after initialisation

```bash
export JIT=true
cargo bench
export JIT=false
cargo bench
```
