
# Initization

1. Database
1.1 relude
docker

brew install libpq
sudo find / -name pg_config
==> $/usr/local/Cellar/libpq/13.2/bin/pg_config
Add to PATH



1.2 install cli
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel setup

diesel migration generate create_users

diesel migration run

diesel migration redo
