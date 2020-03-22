CREATE TABLE posts
(
  -- SERIAL is a PostgreSQL type
  -- INTEGER would be used if using SQLite
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)