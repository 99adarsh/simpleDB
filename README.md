## Building SimpleDB - A simple database engine based on LSM tree and SSTables

The simpleDB is a database engine which is based on LSM Tree to store key-value pairs.
Examples of such database in production are LevelDB, RocksDB, Cassandra etc.

This repository has two parts -
1. SimpelDB database-engine
2. `simple CLI` to use this database engine


### 1. SimpleDB database-engine

`Database-engine` is divided into different parts for ease in development-
<!-- 1. Database frame - It is an abstraction layer of our db-engine wich will be used to interact with the database. Like   -->
1. MemTable - It is an in-memory data-structure which store the key-value pairs in sorted order(sorted by keys). It is the most recent data inserted by users. We will use Red-Black tree to keep the MemTable data sorted all the time.
2. Database - This part will handle the logic for handling the data-store in the disk. i.e. storing and retrieving files from the disk.
3. SSTable - Convert the Memtable data into SSTable and store it the disk using the logic from database.
4. Compaction - Write algorithm for merging logs stored in the disk.


#### Progress and timeline

1. Implement RedBlack tree - Implementation and testing, 30-31 Jan 2024 
2. Implement Memtable - Impl and testing, 1-2 February
3. Implement database, to store and retrieve data from the disk. Impl and testing 3-6 February
4. Convert memtable data into SStable and insert into tree - Impl and testing 7-10 Feb
5. Compaction algorithm - Complete eerything till 15 February
