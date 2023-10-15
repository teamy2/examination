-- Your SQL goes here

CREATE TABLE "completion" (
	"quiz" INT NOT NULL,
	"user" INT NOT NULL,
	"score" SMALLINT NOT NULL,

	PRIMARY KEY("quiz", "user")
);
