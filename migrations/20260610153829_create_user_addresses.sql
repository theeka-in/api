CREATE TABLE user_addresses (
    "id"               UUID NOT NULL,
    "name"             TEXT NOT NULL,
    "complete_address" TEXT NOT NULL,
    "city"             TEXT NOT NULL,
    "state"            TEXT NOT NULL,
    "pincode"          INTEGER NOT NULL,
    "latitude"         DOUBLE PRECISION NOT NULL,
    "longitude"        DOUBLE PRECISION NOT NULL,
    "user_id"          UUID NOT NULL,
    CONSTRAINT user_addresses_pkey PRIMARY KEY ("id"),
    CONSTRAINT user_addresses_user_id_fkey FOREIGN KEY ("user_id") REFERENCES users("id") ON DELETE CASCADE ON UPDATE CASCADE
);
