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
