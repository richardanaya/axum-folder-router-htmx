build:
	cargo build

# ask user for migration name when this is run
create_migration:
	sqlx migrate add -r "$(shell read -p "Enter migration name: " name; echo $$name)"

# run migration using sqlx (cargo install sqlx-cli --no-default-features --features native-tls,postgres)
migrate:
	sqlx migrate run

revert_migrate:
	sqlx migrate revert

ai:
	aider --model openrouter/google/gemini-2.5-pro-preview-03-25 --edit-format diff-fenced