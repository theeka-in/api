CREATE TABLE product_listings (
    "id"    UUID NOT NULL,
    "price" DOUBLE PRECISION NOT NULL,
    "stock" INTEGER NOT NULL,
    CONSTRAINT product_listings_pkey PRIMARY KEY ("id")
);
