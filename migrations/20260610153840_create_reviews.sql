CREATE TABLE reviews (
    "id"          UUID NOT NULL,
    "rating"      INTEGER NOT NULL,
    "title"       TEXT NOT NULL,
    "comment"     TEXT NOT NULL,
    "created_at"  TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id"     UUID NOT NULL,
    "business_id" UUID NOT NULL,
    "listing_id"  UUID NOT NULL,
    CONSTRAINT reviews_pkey PRIMARY KEY ("id"),
    CONSTRAINT reviews_user_id_fkey     FOREIGN KEY ("user_id")     REFERENCES users("id")             ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT reviews_business_id_fkey FOREIGN KEY ("business_id") REFERENCES businesses("id")        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT reviews_listing_id_fkey  FOREIGN KEY ("listing_id")  REFERENCES business_listings("id") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX reviews_user_id_listing_id_key ON reviews("user_id", "listing_id");
