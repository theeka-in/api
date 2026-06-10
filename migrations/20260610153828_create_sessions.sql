CREATE TABLE sessions (
    "token"      TEXT NOT NULL,
    "user_agent" TEXT NOT NULL,
    "ip_address" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "account_id" UUID NOT NULL,
    "user_id"    UUID NOT NULL,
    CONSTRAINT sessions_pkey PRIMARY KEY ("token"),
    CONSTRAINT sessions_account_id_fkey FOREIGN KEY ("account_id") REFERENCES accounts("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT sessions_user_id_fkey    FOREIGN KEY ("user_id")    REFERENCES users("id")    ON DELETE CASCADE ON UPDATE CASCADE
);
