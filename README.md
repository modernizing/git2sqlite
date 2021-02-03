# Git to SQLite

A research on how to save Git log to database for [Coco](https://github.com/inherd/coco)

Architecture

1. parse git log
2. parse log by line to commit
3. save commit to json file
4. read json file and save to db
