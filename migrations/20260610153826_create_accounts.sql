CREATE TABLE accounts (
    "id"         UUID NOT NULL,
    "phone"      BIGINT NOT NULL,
    "password"   TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT accounts_pkey PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX accounts_phone_key ON accounts("phone");
