CREATE TABLE views (
    "id"          UUID NOT NULL,
    "created_at"  TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id"     UUID NOT NULL,
    "business_id" UUID NOT NULL,
    "listing_id"  UUID NOT NULL,
    CONSTRAINT views_pkey PRIMARY KEY ("id"),
    CONSTRAINT views_user_id_fkey     FOREIGN KEY ("user_id")     REFERENCES users("id")             ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT views_business_id_fkey FOREIGN KEY ("business_id") REFERENCES businesses("id")        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT views_listing_id_fkey  FOREIGN KEY ("listing_id")  REFERENCES business_listings("id") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX views_user_id_listing_id_key ON views("user_id", "listing_id");
