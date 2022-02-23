-- DEV ONLY - Comment out for keeping db beween restart
DROP DATABASE IF EXISTS app_db; -- Drop database if it exists
DROP USER IF EXISTS app_user; -- Drop user if it exists

-- DEV ONLY - for quick insertion
CREATE USER app_user PASSWORD 'app_pwd_to_change'; -- create user app_user with password app_user
CREATE DATABASE app_db owner app_user ENCODING 'UTF-8'; -- create db app_db with owner app_user
