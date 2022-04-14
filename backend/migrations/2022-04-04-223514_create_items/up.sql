CREATE TABLE "items" (
	"name"	TEXT NOT NULL UNIQUE,
	"price"	INTEGER NOT NULL CHECK ("price" >= 0),
	"tax"	REAL NOT NULL,
	"type" TEXT NOT NULL CHECK ("type" IN ("food", "drink", "other")),
	PRIMARY KEY("name")
);