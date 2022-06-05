# config

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
