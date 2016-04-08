# Client Performance Tests

This repo is an unscientific attempt to benchmark the performance of a few Elasticsearch clients.
Results are really just an indication of the amount of work a particular client on a particular
platform needs to do to send a single search request to Elasticsearch and deserialise the results into some native form.

This is only benchmarking the time it takes to send a single request, so things like connection pools don't come into the equation, even though they're super-important.

It would be great to have a _real_ test dataset that can be used for a more realistic set of
benchmarks including concurrent queries and doc indexing. This will do for now though.

## Process

The test data is built with the following Sense script:

```
PUT bench_index

PUT bench_index/bench_doc/_mapping
{
  "properties": {
    "id": {
      "type": "integer"
    },
    "title": {
      "type": "string"
    },
    "timestamp": {
      "type": "date",
      "format": "epoch_millis"
    }
  }
}

PUT bench_index/bench_doc/1
{
  "id": 1,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/2
{
  "id": 2,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/3
{
  "id": 3,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/4
{
  "id": 4,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/5
{
  "id": 5,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/6
{
  "id": 6,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/7
{
  "id": 7,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/8
{
  "id": 8,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/9
{
  "id": 9,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/10
{
  "id": 10,
  "title": "Document",
  "timestamp": 1460002100704
}
```

The test request is as follows:

```
POST bench_index/bench_doc/_search
{
  "query": {
    "query_string": {
      "query": "*"
    }
  },
  "size": 10
}
```

A baseline that gives the approximate time taken to get a response from Elasticsearch is produced
using `ab`:

## Results

### Baseline

```
ab -n 200 -c 1 -T 'application/json' -p postdata.txt http://localhost:9200/bench_index/bench_doc/_search

Time per request:       1.298 [ms] (mean)
                        1421.67 kb/s total

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      1
  80%      1
  90%      2
  95%      2
  98%      4
  99%      9
 100%     10 (longest request)
```

### Rust (rs-es)

```
Time per request:       2.309447 [ms] (mean)

Percentage of the requests served within a certain time (ms)
  50%      2.219113
  66%      2.231041
  75%      2.242552
  80%      2.249886
  90%      2.277720
  95%      2.298054
  98%      2.349468
  99%      2.413538
 100%     18.555058 (longest request)
```

### Rust (elastic_hyper + elastic_types)

```
Time per request:       2.335910 [ms] (mean)

Percentage of the requests served within a certain time (ms)
  50%      2.284115
  66%      2.297106
  75%      2.302009
  80%      2.308742
  90%      2.336854
  95%      2.367203
  98%      2.399045
  99%      2.799395
 100%     10.003992 (longest request)
```

### Rust (elastic_hyper + custom)

*NOTE:* This implementation breaks the rules by specifying a `filter_path` on the return set. So there are actually fewer bits being returned.

The point of this one is to show that `elastic_hyper` basically sits right on top of the minimum request time possible, and any work you do to the response is up to you.

```
Time per request:       1.573568 [ms] (mean)

Percentage of the requests served within a certain time (ms)
  50%      1.512144
  66%      1.529050
  75%      1.538743
  80%      1.545430
  90%      1.571101
  95%      1.757354
  98%      2.467757
  99%      2.849229
 100%      4.878057 (longest request)
```

### CSharp (Elasticsearch.NET)

```
Time per request:       3.023890 [ms] (mean)

Percentage of the requests served within a certain time (ms)
  50%      1.708400
  66%      1.766000
  75%      1.817200
  80%      1.939000
  90%      2.547900
  95%      2.643800
  98%      3.322400
  99%      6.002800
 100%    224.919900 (longest request)
```