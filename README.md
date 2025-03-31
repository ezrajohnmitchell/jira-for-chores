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
this project uses sqlx compile time query checking, ensure that a database with migrations is avaiable by running before build

```bash
docker compose -f db.yml up
```
```bash
sqlx migration run
```
Working on enabling offline mode once more of the app is finished