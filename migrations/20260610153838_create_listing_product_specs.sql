CREATE TABLE listing_product_specs (
    "id"                 UUID NOT NULL,
    "group_name"         TEXT NOT NULL,
    "key"                TEXT NOT NULL,
    "value"              TEXT NOT NULL,
    "product_listing_id" UUID NOT NULL,
    CONSTRAINT listing_product_specs_pkey PRIMARY KEY ("id"),
    CONSTRAINT listing_product_specs_product_listing_id_fkey FOREIGN KEY ("product_listing_id") REFERENCES product_listings("id") ON DELETE CASCADE ON UPDATE CASCADE
);
