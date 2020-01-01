FROM postgres:alpine

RUN apk add --no-cache postgresql-contrib

ADD ./app/migrations/00000000000000_diesel_initial_setup/up.sql /docker-entrypoint-initdb.d/01_init_database.sql
ADD ./app/migrations/2018-12-09-163031_create_posts/up.sql /docker-entrypoint-initdb.d/02_create_table.sql


EXPOSE 5432
