-- DEV ONLY - Comment out for keeping db beween restart
DROP DATABASE IF EXISTS pos_db; -- Drop database if it exists
DROP USER IF EXISTS pos_user; -- Drop user if it exists

-- DEV ONLY - for quick insertion
CREATE USER pos_user PASSWORD 'pos_password'; -- create user app_user with password app_user
CREATE DATABASE pos_db owner pos_user ENCODING 'UTF-8'; -- create db app_db with owner app_user