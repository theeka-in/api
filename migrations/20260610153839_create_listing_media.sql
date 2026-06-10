CREATE TABLE listing_media (
    "id"         UUID NOT NULL,
    "type"       media_type NOT NULL,
    "url"        TEXT NOT NULL,
    "listing_id" UUID NOT NULL,
    CONSTRAINT listing_media_pkey PRIMARY KEY ("id"),
    CONSTRAINT listing_media_listing_id_fkey FOREIGN KEY ("listing_id") REFERENCES business_listings("id") ON DELETE CASCADE ON UPDATE CASCADE
);
