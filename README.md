# FerriteDB

FerriteDB is an experimental relational database engine written in Rust.

## Features
- SQL parsing
- In-memory data storage
- Configurable row-based or columnar storage (planned)
- JSON-based catalog storage (since v0.4.0)

## Installation
Clone the repository:
```bash
git clone https://github.com/yourusername/ferritedb.git
```

Run FerriteDB
```bash
cd ferrite-db
cargo run
```

Create a table
```bash
Welcome to ferrite-db!
> create table popular_databases (id int, name text, rank int);
Table created: popular_databases
```

Insert into a table
```bash
> insert into popular_databases values (1, Oracle, 1);
Inserted into popular_databases
> insert into popular_databases values (2, MySQL, 2);
Inserted into popular_databases
> insert into popular_databases values (3, PostgreSQL, 3);
Inserted into popular_databases
```

Select from a table
```bash
> select * from popular_databases;
["1", "Oracle", "1"]
["2", "MySQL", "2"]
["3", "PostgreSQL", "3"]
```

View the catalog.json file
```json
{
  "popular_databases": {
    "name": "popular_databases",
    "columns": [
      {
        "name": "id",
        "data_type": "INT"
      },
      {
        "name": "name",
        "data_type": "TEXT"
      },
      {
        "name": "rank",
        "data_type": "INT"
      }
    ]
  }
}
```