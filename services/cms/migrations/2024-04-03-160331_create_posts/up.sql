-- Your SQL goes here
CREATE TABLE "posts"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL,
	"slug" TEXT NOT NULL,
	"content" TEXT NOT NULL,
	"published" BOOL NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"created_by" TEXT NOT NULL,
	"last_modified_at" TIMESTAMP NOT NULL,
	"last_modified_by" TEXT NOT NULL
);

