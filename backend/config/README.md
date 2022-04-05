# config
## in pos_config.toml
in `pos_config.toml` add changes to the backend the default values are:

## default values
```toml
[database]
net_id = "0.0.0.0"
port = 5432
max_connections = 5
root_db_name = "postgres"
root_user = "postgres"
root_pwd = "postgres"
db_name = "pos_db"
user = "pos_user"
pwd = "pos_user"
file_path = "pos_db.db"

[webserver]
net_id = "0.0.0.0"
port = 8080
folder = "../frontend/web-folder/"
```

## example config
```toml
[database]
root_user = "root"
root_pwd = "root"

[webserver]
port = 3030
```
this will only change the database's `root_user` and `root_pwd` and the webserver's `port` all other values will be defaulted to