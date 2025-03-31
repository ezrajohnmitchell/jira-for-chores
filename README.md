# Jira-For-Chores
A task management app meant for use in a household. The starting implementation contains some simple ways to categorize workers with more planned

## Source Layout
```
+---core //library only
|   +---catalogue  //crud portion of app, manages chore catalogue
|   |   | infrastructure //db
|   |   | service 
|   +---management //manages task status/assignment, 
|   |   | application //use cases
|   |   | models //domain objects
|   |   | infrastructure //db
+---web //maps relevant use cases from core to http endpoints
+---batch //uses core to perform daily operations
```

## Building locally
this project uses sqlx compile time query checking, to change or add queries an active database connection is required. Use the following to run a postgres container and configure sqlx

```bash
docker compose -f db.yml up
```
then set the environment variable `DATABASE_URL=postgres://admin:admin123@localhost/local`
```bash
sqlx migration run
```
At this point builds will check the queries against the running database


to reenable offline building run 
```bash
cargo sqlx prepare --workspace
```