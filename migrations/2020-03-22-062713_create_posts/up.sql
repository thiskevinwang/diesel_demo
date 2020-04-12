CREATE TABLE posts
(
  -- SERIAL is a PostgreSQL type
  -- INTEGER would be used if using SQLite
  id SERIAL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  -- 'f' equals false
  published BOOLEAN NOT NULL DEFAULT 'f',
  CONSTRAINT "PK_posts_id" PRIMARY KEY ("id")
)