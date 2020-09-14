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

Finally load the data with the following SQL statement:

```sql
LOAD DATA
    LOCAL
    INFILE 'a.sql'
    INTO TABLE c
    FIELDS TERMINATED BY ','
    OPTIONALLY ENCLOSED BY '"'
    (`num`, `subnum`, `thread_num`, `op`, `timestamp`, `timestamp_expired`, `preview_orig`, `preview_w`, `preview_h`, `media_filename`, `media_w`, `media_h`, `media_size`, `media_hash`, `media_orig`, `spoiler`, `deleted`, `capcode`, `email`, `name`, `trip`, `title`, `comment`, `sticky`, `locked`, `poster_hash`, `poster_country`, `exif`, @timestamp)
    SET
        unix_timestamp=FROM_UNIXTIME(@timestamp);
```