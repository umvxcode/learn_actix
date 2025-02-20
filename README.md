
# Rust Actix + MySQL


Rust MySQL Example Project
This is a simple Rust project that demonstrates how to connect to a MySQL database running on localhost and expose a REST API on port 8080. The project uses the mysql crate for database access and warp for handling HTTP requests.

Prerequisites
Before running the project, ensure you have the following installed:

Rust: Install Rust from rustup.rs.

MySQL: Install MySQL and ensure it is running on localhost. You should have a database and a table set up.

Cargo: Rust's package manager, which comes with Rust.


## Installation

```bash
git clone https://github.com/umvxcode/learn_actix.git
cd learn_actix
```
## Create Dababase

```bash

CREATE DATABASE IF NOT EXISTS `latihanrust`

USE `latihanrust`;


CREATE TABLE IF NOT EXISTS `buku` (
  `id` int NOT NULL AUTO_INCREMENT,
  `tahun_terbit` int DEFAULT NULL,
  `penulis` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `judul` varchar(250) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

```
## Runing

run Project

```bash
cargo watch -x run
```


## API Reference

#### Get all items

```http
  GET /buku/index
```

#### Create item

```http
  POST /api/buku/create
  Type json
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `tahun_terbit`      | `int` | **Required**. |
| `penulis`      | `string` | **Required**. |
| `judul`      | `string` | **Required**. |


#### Get one item

```http
  GET /api/buku/byid/${id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `int` | **Required**. Id of item to fetch |


#### Update item

```http
  POST /api/buku/update
  Type json
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `int` | **Required**. |
| `tahun_terbit`      | `int` | **Required**. |
| `penulis`      | `String` | **Required**. |
| `judul`      | `String` | **Required**. |


#### Get one item

```http
  GET /api/buku/delete/${id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `int` | **Required** |

## Authors

- [@umvxcode](https://www.github.com/umvxcode)

