CREATE TABLE service_listings (
    "id"        UUID NOT NULL,
    "price"     TEXT NOT NULL,
    "available" BOOLEAN NOT NULL,
    CONSTRAINT service_listings_pkey PRIMARY KEY ("id")
);
