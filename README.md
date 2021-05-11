# Groceries API

TODO lists as starter projects are everywhere, which is why I made something completely different. 

This API manages items users need to buy at the grocery store. Authentication is handled with JWTs and users can only see their own grocery list.

This is still very much a work in progress.

## Getting started

Use docker compose to start a postgres server:
```
docker compose up -d
```

Export environment variables:
```
export DATABASE_URL=postgres://admin:admin@0.0.0.0:5432
export ROCKET_ADDRESS=0.0.0.0
export ROCKET_PORT=8000
```

Create a secret key for JWTs:
```
Rename src/secret.key.sample to src/secret.key
```
