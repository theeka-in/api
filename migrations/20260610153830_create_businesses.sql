CREATE TABLE businesses (
    "id"           UUID NOT NULL,
    "phone_number" BIGINT NOT NULL,
    "is_closed"    BOOLEAN NOT NULL DEFAULT false,
    "title"        TEXT NOT NULL,
    "logo"         TEXT,
    "description"  TEXT,
    "created_at"   TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "owner_id"     UUID NOT NULL,
    CONSTRAINT businesses_pkey PRIMARY KEY ("id"),
    CONSTRAINT businesses_owner_id_fkey FOREIGN KEY ("owner_id") REFERENCES users("id") ON DELETE CASCADE ON UPDATE CASCADE
);
