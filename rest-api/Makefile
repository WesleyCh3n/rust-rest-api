host     := localhost
user     := wesley
db       := testdb
password := 970530
port     := 5432
sslmode  := disable

ctr:
	docker run --name postgres_ctr -p $(port):$(port) \
		-e POSTGRES_USER=$(user) \
		-e POSTGRES_PASSWORD=$(password) \
		-d postgres

start:
	docker start postgres_ctr

stop:
	docker stop postgres_ctr

createdb:
	docker exec -it postgres_ctr createdb --username=$(user) --owner=$(user) $(db)

dropdb:
	docker exec -it postgres_ctr dropdb $(db)

shell:
	docker exec -it postgres_ctr psql -U $(user) -h 127.0.0.1 -p $(port)

remove_ctr:
	docker stop postgres_ctr
	docker rm postgres_ctr
