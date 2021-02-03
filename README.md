# Git to SQLite

A research on how to save Git log to database for [Coco](https://github.com/inherd/coco)

Architecture

1. parse git log
2. parse log by line to commit
3. save commit to json file
4. read json file and save to db

## Performance logs

### Without Changes

- Machine: MacBook Pro (15-inch, 2018)
- Processor: 2.2 GHz 6-Core Intel Core i7
- Memory: 16 GB 2400 MHz DDR4

| Project Name     | Project Commits | Time   | Times(ms)         |
|------------------|-----------------|--------|-------------------|
| Rust Regex       | 1078            | 3s     | 2919ms ~ 3012ms   |
| Lombok           | 3127            | 8s     | 8096ms ~ 8616ms   |
| Nginx            | 6805            | 32s    | 32468ms ~ 33967ms |
| Redis            | 10009           | 67s    | 65328ms ~ 71616ms |
| Spring Framework | 22133           | 706s   |                   |
| Graal            | 49026           | 1425s  |                   |
| Gradle           | 78711           | 4130s  |                   |


License
---

@ 2020~2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
