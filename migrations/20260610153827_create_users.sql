CREATE TABLE users (
    "id"         UUID NOT NULL,
    "name"       TEXT NOT NULL,
    "avatar"     TEXT,
    "account_id" UUID NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY ("id"),
    CONSTRAINT users_account_id_fkey FOREIGN KEY ("account_id") REFERENCES accounts("id") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX users_account_id_key ON users("account_id");
