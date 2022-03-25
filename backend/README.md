# about
this is the backend of the point of sale (pos) system.

the goal of this code is to have a point of sale system where a business is able to have employees press icons that represent items when checking some one else.

some busineeses have employees memorize all the prices of goods and then manually enter them into the cash register, but that requires more time training their employees and it could lead to the employees entering the wrong prices for goods.

instead of relying on employees memory which can result in error and loss of sales it is more efficient for them to just press the icon for the coresponding items.

once the has entered all of the required items, they will be able to press complete transation which will tell them how much the sale was and enter the sale in a database.

there will be an admin pannel where the database entries can be deleted or altered and there will be an option to allow cash tranations to be added in the sale database.

# start
## start the database using docker-compose
```sh
sudo docker-compose up
```
this works because it uses the `docker-compose.yml` file

## start the backend
```sh
cargo run --release
```

## config files
the code currently uses config files located at `./config/`

diesel uses a `.env` file for its cli utils while developing, this file can be ignored, but if you are developing you need to make sure the file is properly updated
# testing
```sh
sudo docker-compose up                                    # start the docker container
sh generate_test_seeds.sh                                 # generate test seeds to the database
cargo test -- --test-threads=1 --nocapture --color=always # run the tests
sh remove_test_seeds.sh                                   # remove the seeds from the database
```
