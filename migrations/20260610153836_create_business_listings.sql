CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE business_listings (
    "id"                 UUID NOT NULL,
    "title"              TEXT NOT NULL,
    "description"        TEXT,
    "logo"               TEXT,
    "is_active"          BOOLEAN NOT NULL DEFAULT true,
    "created_at"         TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at"         TIMESTAMP(3) NOT NULL,
    "business_id"        UUID NOT NULL,
    "product_listing_id" UUID,
    "service_listing_id" UUID,
    "embedding"          vector(1024) NOT NULL,
    CONSTRAINT business_listings_pkey PRIMARY KEY ("id"),
    CONSTRAINT business_listings_business_id_fkey        FOREIGN KEY ("business_id")        REFERENCES businesses("id")       ON DELETE CASCADE  ON UPDATE CASCADE,
    CONSTRAINT business_listings_product_listing_id_fkey FOREIGN KEY ("product_listing_id") REFERENCES product_listings("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT business_listings_service_listing_id_fkey FOREIGN KEY ("service_listing_id") REFERENCES service_listings("id") ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE UNIQUE INDEX business_listings_product_listing_id_key ON business_listings("product_listing_id");
CREATE UNIQUE INDEX business_listings_service_listing_id_key ON business_listings("service_listing_id");
