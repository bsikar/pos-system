# config
## webserver
in `webserver.toml` (example config)
```toml
[webserver]
net_id = "0.0.0.0"
port = 3030
folder = "web-folder/"
```
if you don't put some of the values for the variables defualt ones will be used


the default values are:
```toml
net_id = "0.0.0.0"
port = 8080
folder = "../frontend/web-folder/"
```
### net_id -- default value: `"0.0.0.0"`
this is the ip address to connect to, e.g `127.0.0.1` or `0.0.0.0`
### web_port -- default value: `8080`
this is the port that the server will bind to
### web_folder -- default value: `"../frontend/web-folder/"`
this is the servers front end files

## database
in `database.toml` (example config)
```toml
[database]
max_connections = 3
root_db_name = "postgres"
root_pwd = "root password"
db_name = "DATABASE"
user = "USER"
pwd = "PASSWORD"
```
if you don't put some of the values for the variables defualt ones will be used


the default values are:
```toml
net_id = "0.0.0.0"
port = 5432
max_connections = 5
root_db_name = "postgres"
root_user = "postgres"
root_pwd = "postgres"
db_name = "postgres"
user = "pos_user"
pwd = "pos_user"
```
### net_id -- default value: `"0.0.0.0"`
this is the ip address to connect to, e.g `127.0.0.1` or `0.0.0.0`
### port -- default value: `5432`
this is the port that the database will bind to
### max_connections -- default value: `5`
this is the max number of connections to the database at one time
### root_db_name -- default value: `"postgres"`
this is the name of the root database
### root_user -- default value: `"postgres"`
this is the root users name
### root_pwd -- default value: `"postgres"`
this is the password for the root user
### db_name -- default value: `"pos_db"`
this is the primary database name
### user -- default value: `"pos_user"`
this is the name of the primary user
### pwd -- default value: `"pos_user"`
this is the password for the primary user