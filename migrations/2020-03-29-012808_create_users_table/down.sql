ALTER TABLE "posts"
DROP CONSTRAINT "FK_posts_user_id";

ALTER TABLE "posts"
DROP COLUMN "user_id";

DROP TABLE "users";