# about
this is the backend of the point of sale (pos) system.

the goal of this code is to have a point of sale system where a business is able to have employees press icons that represent items when checking some one else.

some busineeses have employees memorize all the prices of goods and then manually enter them into the cash register, but that requires more time training their employees and it could lead to the employees entering the wrong prices for goods.

instead of relying on employees memory which can result in error and loss of sales it is more efficient for them to just press the icon for the coresponding items.

once the has entered all of the required items, they will be able to press complete transation which will tell them how much the sale was and enter the sale in a database.

there will be an admin pannel where the database entries can be deleted or altered and there will be an option to allow cash tranations to be added in the sale database.

# start
## start the database using docker-compose (easiest)
```sh
sudo docker-compose up
```
this works because it uses the `docker-compose.yml` file

## start the database using docker cli
to start the database run the following command:
```sh
sudo docker run --rm \
	--name pos-pg \
	-v $PWD/postgres-data/:/var/lib/postgresql/data \
	-p 5432:5432 \
	-e POSTGRES_PASSWORD=postgres_password \
	-e POSTGRES_USER=postgres_user \
	-e POSTGRES_DB=postgres_db \
	postgres:14
```
heres the break down of what that command does:

`run` : start an instance of a docker container

`--rm` :  removes a container after it exits

`--name pos-pg` : assign the name `pos-pg` to the container (`pos-pg` meaning "point of sale postgres")

`-v <host directory>:<container directory>`

`-v $PWD/postgres-data/:/var/lib/postgresql/data` :  put the data from the container to the local host

`-p <container's TCP port>:<host's TCP port>`

`-p 5432:5432 ` : bind the container's TCP port `5432` to the host's port `5432`

the port of `5432` is usually the default port for postgres

`-e "POSTGRES_PASSWORD=postgres"` : set the enviromental variable `POSTGRES_PASSWORD` to `postgres`


`postgres:14` : use the [docker image](https://hub.docker.com/_/postgres) `posgres` version `14`

## to check the database
to check the contents of the database and to preform maintance on the database run the following command:
```sh
docker exec -i -t -u postgres pos-pg psql
```
heres the break down of what that command does:

`exec` : run a command in a running docker container

`-i` : keep STDIN open even if its not attached meaning make it interactive

`-t` : allocate a pseudo-TTY

`-u postgres` : access with username `postgres`

`pos-pg` : the docker container is called `pos-pg`

`psql` : the command is called `psql`

`psql` is a command which allows us to access and mess with the database

# development
## run tests
to test the code (run tests) run the following command
```sh
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture --color=always'
```
heres the break down of waht that command does:

`-q` : suppress output from cargo-watch

`-c` : clear the screen before each run

`-w src/` : watch the `src/` directory

`-x <command>` : run the following cargo command to execute changes

the command: `test model_db_ -- --test-threads=1 --nocapture`

`test model_` : run the tests that start with the name `model_`

`--` : this is needed before the test flags to run the application with those flags

`--test-threads=1` : use one thread when testing

`--nocapture` : show output of the test

`--color=always` : enable color in the output

you can run the web tests in the same way
```sh
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture --color=always'
```

to run all tests
```sh
cargo watch -q -c -w src/ -x 'test -- --test-threads=1 --nocapture --color=always'
```

## start the website
```sh
cargo watch -q -c -w src/ -x 'run -- -i ../frontend/web-folder -f actix -p 3030'
```
heres the break down of waht that command does:

`-q` : suppress output from cargo-watch

`-c` : clear the screen before each run

`-w src/` : watch the `src/` directory

`-x <command>` : run the following cargo command to execute changes

the command: `test model_db_ -- --test-threads=1 --nocapture`

`test model_` : run the tests that start with the name `model_`

`--` : this is needed before the test flags to run the application with those flags

`-i ../frontend/web-folder` : this is the path to the webfolder which will be used

`-f actix` : this is the web framework that the server will use

`-p 3030` : bind the http server to port `3030`