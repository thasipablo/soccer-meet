DB_DOCKER_CONTAINER=actix-web-postgres

run:
	cargo run

build:
	cargo build

create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d -p 5432:5432 postgres:16.3-bullseye

create_postgres_db:
	docker exec -it ${DB_DOCKER_CONTAINER} createdb --username=root --owner=root soccerrustytdb
