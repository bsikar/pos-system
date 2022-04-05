CREATE TABLE "items" (
	"name"	TEXT NOT NULL UNIQUE,
	"price"	INTEGER NOT NULL CHECK ("price" >= 0),
	"tax"	REAL NOT NULL,
	PRIMARY KEY("name")
);