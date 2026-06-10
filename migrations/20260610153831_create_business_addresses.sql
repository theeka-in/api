CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE business_addresses (
    "business_id"   UUID NOT NULL,
    "address_line1" TEXT NOT NULL,
    "address_line2" TEXT NOT NULL,
    "landmark"      TEXT,
    "city"          TEXT NOT NULL,
    "state"         TEXT NOT NULL,
    "pincode"       TEXT NOT NULL,
    "radius"        DOUBLE PRECISION NOT NULL,
    "location"      geometry(Point, 4326) NOT NULL,
    CONSTRAINT business_addresses_pkey PRIMARY KEY ("business_id"),
    CONSTRAINT business_addresses_business_id_fkey FOREIGN KEY ("business_id") REFERENCES businesses("id") ON DELETE CASCADE ON UPDATE CASCADE
);
