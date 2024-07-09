To launch the backend app:
  cd <REPOSITORY_ROOT>/backend
  cargo watch -x run

Works Cited:
https://github.com/brooks-builds/full-stack-todo-rust-course/tree/main/backend/rust/axum

Please do not forget about the doc James.
I go to AI and Google way too fast a lot of the time.
Look at the docs:
https://docs.rs/axum/latest/axum/
https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/

Sea ORM init:
cd /Users/jameshawleyii/MyCodingProjects/RustPractice/practice-data-processing/backend/src/entities
sea-orm-cli generate entity -o src/database
sea-orm-cli generate entity -o src/test_entities
sea-orm-cli generate entity -o .

echo 'export PATH="/usr/local/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc

psql -h <hostname> -p <port> -U <username> -d <database>
psql -h localhost -p 5432 -U postgres -d postgres

docker-compose -f docker-compose_2.yml up -d
docker-compose down


































