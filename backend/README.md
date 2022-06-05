# about
this is the backend of the point of sale (pos) system.

the goal of this code is to have a point of sale system where a business is able to have employees press icons that represent items when checking some one else.

some businesses have employees memorize all the prices of goods and then manually enter them into the cash register, but that requires more time training their employees and it could lead to the employees entering the wrong prices for goods.

instead of relying on employees memory which can result in error and loss of sales it is more efficient for them to just press the icon for the corresponding items.

once the has entered all of the required items, they will be able to press complete transactions which will tell them how much the sale was and enter the sale in a database.

there will be an admin panel where the database entries can be deleted or altered and there will be an option to allow cash transactions to be added in the sale database.

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install pos-system`

## License

Licensed under  MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).

## start the backend
make `<database>.db`
```sh
diesel migration run
```
delete and make new `<database>.db` * **you will lose all your database data**
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

# development
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

# testing
```sh
cd scripts
pip install -r requirements.txt
python test-backend.py
```
this will start the server and run the tests by adding seed data to the database

# Common Problems
* make sure that the ports you are binding to are not already in use
* make sure you have sqlite installed
```sh
sudo apt install libsqlite3-dev
```