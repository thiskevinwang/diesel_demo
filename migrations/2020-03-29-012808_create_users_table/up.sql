-- Constraint naming 1
-- https://til.cybertec-postgresql.com/post/2019-09-02-Postgres-Constraint-Naming-Convention/
-- Constraint name 2
-- https://www.npmjs.com/package/@z-brain/typeorm-postgres-camelcase-naming-strategy#features
CREATE TABLE "users"
(
  "id" SERIAL NOT NULL,
  "updated_at" TIMESTAMP DEFAULT NOW(),
  "deleted_at" TIMESTAMP,
  "name" CHARACTER VARYING(55),
  "username" CHARACTER VARYING(55),
  CONSTRAINT "UQ_users_username" UNIQUE("username"),
  CONSTRAINT "PK_users_id" PRIMARY KEY ("id")
);

ALTER TABLE "posts" 
ADD "user_id" INTEGER;

ALTER TABLE "posts"
ADD CONSTRAINT "FK_posts_user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
