Concurrency Level: 100
Time taken for tests: 1.994 seconds
Complete requests: 10000
Failed requests: 0
Total transferred: 2870000 bytes
HTML transferred: 1600000 bytes
Requests per second: 5015.24 [#/sec] (mean)
Time per request: 19.939 [ms] (mean)
Time per request: 0.199 [ms] (mean, across all concurrent requests)
Transfer rate: 1405.64 [Kbytes/sec] received

ab -c 100 -n 10000 http://127.0.0.1:8000/

concurrency 100
requests 10000

A LOT FASTER WITH ASYNC RUNTIME
