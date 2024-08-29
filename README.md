# Benchmark

## Test configurations
Print 500,000 logs. Perform the test 5 times. Before each test, sleep for 2 seconds, then print a warm-up message, and then continuously print 500,000 messages. Test has been done on two types: i32 and

```Rust
struct LogStruct {
    data: [u64; 10],
}
```
## message examples
### flashlog: [test-file](./examples/flashlog/src/main.rs)
```Json
{"data":{"test_struct":{"data":[1,2,3,4,5,6,7,8,9,10]}},"date":"20240829","level":"Info","offset":9,"src":"src/main.rs:48","time":"09:22:14.328:084:318","topic":"Log message"}
```

### ftlog: [test-file](./examples/ftlog/src/main.rs)
```
2024-08-29 09:39:13.503+09 0ms INFO main [src/main.rs:57] Log message: LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
```
## Performance comparisons
### Test machine: i5-14400F, 2.5Ghz

| Logger    | i32           | 80 byte struct  |
| --------- | ------------- | --------------- |
| flashlog  | 80 ns         | 90 ns                 |
| ftlog     | 102 ns        | 107 ns                 |
| fast_log  | 98 ns                     | 103 ns                 |
