# chump2csv

This is a tool to convert dumps created by [mysqlchump](https://github.com/bbepis/mysqlchump) to CSV files that be loaded into MySQL with the [LOAD DATA](https://dev.mysql.com/doc/refman/8.0/en/load-data.html) statement for performance.

## Usage

Using `bzcat` and `pv` (optional) you can generate a sql dump which you can then load into MySQL.

```sh
$ bzcat a.sql.bz2 | pv | chump2csv > a.csv
```

By default, chump2csv will also include the unix_timestamp field, used by [Torako](http://github.com/miyachan/torako). That can be disabled for support with just Asagi.

```sh
$ bzcat a.sql.bz2 | chump2csv --no_unix_timestamp > a.csv
```

You can also precompute stats tables as a CSV.

```sh
$ bzcat a.sql | chump2csv --threads a_threads.sql --users a_users.sql --images a_images.sql > a.csv
```

If you plan to import into something like sqlite, you may want to use tab seperated columns:

```sh
$ bzcat a.sql | chump2csv --delimiter tab > a.tsv
```

#### Full example

```
$ pv -cN source a.sql.bz2 | bzcat | pv -cN bzcat | ./chump2csv --images a/a_images.csv --daily a/a_daily.csv --threads a/a_threads.csv --users a/a_users.csv | pv -cN chump2csv > a/a.csv
```

Finally load the data with the following SQL statement:

```sql
LOAD DATA
    LOCAL
    INFILE 'a.csv'
    INTO TABLE a
    FIELDS TERMINATED BY ','
    OPTIONALLY ENCLOSED BY '"'
    (`num`, `subnum`, `thread_num`, `op`, `timestamp`, `timestamp_expired`, `preview_orig`, `preview_w`, `preview_h`, `media_filename`, `media_w`, `media_h`, `media_size`, `media_hash`, `media_orig`, `spoiler`, `deleted`, `capcode`, `email`, `name`, `trip`, `title`, `comment`, `sticky`, `locked`, `poster_hash`, `poster_country`, `exif`, `media_id`, @timestamp)
    SET
        unix_timestamp=FROM_UNIXTIME(@timestamp);
```

#### Threads

```sql
LOAD DATA
    LOCAL
    INFILE 'a_threads.csv'
    INTO TABLE a_threads
    FIELDS TERMINATED BY ','
    OPTIONALLY ENCLOSED BY '"'
    (`thread_num`, `time_op`, `time_last`, `time_bump`,
    `time_last_modified`, `nreplies`, `nimages`, `sticky`, `locked`);
```

#### Images

```sql
LOAD DATA
    LOCAL
    INFILE 'a_images.csv'
    INTO TABLE a_images
    FIELDS TERMINATED BY ','
    OPTIONALLY ENCLOSED BY '"'
    (`media_id`, `media_hash`, `media`, `preview_op`, `preview_reply`,
    `total`, `banned`);
```

#### Daily

```sql
LOAD DATA
    LOCAL
    INFILE 'a_daily.csv'
    INTO TABLE a_daily
    FIELDS TERMINATED BY ','
    OPTIONALLY ENCLOSED BY '"'
    (`day`, `posts`, `images`, `sage`,
    `anons`, `trips`, `names`);
```

#### Users

```sql
LOAD DATA
    LOCAL
    INFILE 'a_users.csv'
    INTO TABLE a_users
    FIELDS TERMINATED BY ','
    OPTIONALLY ENCLOSED BY '"'
    (`name`, `trip`, `firstseen`, `postcount`);
```

## `board_images` linking

When creating the images CSV, chump2csv will attempt to make sure every post has a valid corresponding `media_id`. If you attempt to load the images into a table that isn't empty, you may incorrectly overwrite data. Instead set the `--images_start_index` flag to some number greater than the most recent auto increment ID. To disable this (set `media_id` to 0), then set `--images_start_index` to 0.