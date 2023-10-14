CREATE TABLE "user" (
    "id" SERIAL PRIMARY KEY,

    "username" VARCHAR(16) UNIQUE NOT NULL,
    "password" BYTEA NOT NULL
);

CREATE TABLE "quiz" (
    "id" SERIAL PRIMARY KEY,
    "author" INT NOT NULL,

    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL
);


CREATE TABLE "question" ( 
    "id" SERIAL PRIMARY KEY,
    "quiz" INT NOT NULL,

    "name" TEXT NOT NULL,
    "options" TEXT[] NOT NULL,
    "answers" SMALLINT[] NOT NULL
);
