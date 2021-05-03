Use docker compose to start a postgres server:
```
docker compose up -d
```

Export the connection url:
```
export DATABASE_URL=postgres://admin:admin@0.0.0.0:5432
```
