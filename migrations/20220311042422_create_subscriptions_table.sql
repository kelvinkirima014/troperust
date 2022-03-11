-- Add migration script here
--create Subscriptions Table
CREATE TABLE Subscriptions(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	subsribed_at timestampz NOT NULL
);