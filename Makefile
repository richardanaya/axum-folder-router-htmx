build:
	cargo build

# ask user for migration name when this is run
create_migration:
	sqlx migrate add -r "$(shell read -p "Enter migration name: " name; echo $$name)"

# run migration using sqlx (cargo install sqlx-cli --no-default-features --features native-tls,postgres)
migrate:
	(env $(cat .env | xargs) sqlx migrate run )