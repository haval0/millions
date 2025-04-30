# millions

Free and easy events/news system for organizations.

## Running

Either set up Postgres on your server or run it with docker as described below.


```sh
docker run -p 5432:5432 --name millions-postgres -e POSTGRES_USER=millions -e POSTGRES_PASSWORD=millions -d postgres
cargo r
```
