# Backend

## Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install pos-system`

## start the backend

make `<database>.db`

```sh
diesel migration run
```

delete and make new `<database>.db` \* **you will lose all your database data**

```sh
diesel migration redo
```

start the server

```sh
cargo run --release
```

## config files

the code currently uses config files located at `./config/`

diesel uses a `.env` file for its cli utils while developing, this file can be ignored, but if you are developing you need to make sure the file is properly updated

## in pos_config.toml

in `pos_config.toml` add changes to the backend the default values are:

## default values

```toml
[database]
file_path = "pos_db.db"

[webserver]
net_id = "0.0.0.0"
port = 8080
folder = "../frontend/web-folder/"
```

## example config

```toml
[database]
file_path = "database.db"

[webserver]
port = 3030
```

this will only change the database's `file_path` and the webserver's `port` all other values will be defaulted to

## development

install dependencies

```sh
cd scripts
bash bootstrap.sh
```

before pushing run the `presubmit.sh` script

```sh
cd scripts
bash presubmit.sh
```

to quickly add data to the database run the `test-db.sh` script

```sh
cd scripts
bash test-db.sh
```

## testing

```sh
cd scripts
pip install -r requirements.txt
python test-backend.py
```

this will start the server and run the tests by adding seed data to the database

## Common Problems

- make sure that the ports you are binding to are not already in use
- make sure you have sqlite installed

```sh
sudo apt install libsqlite3-dev
```
