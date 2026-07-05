ALTER TABLE service_listings
    ADD COLUMN "service_listings_type_id" UUID NOT NULL;
 
ALTER TABLE service_listings
    ADD CONSTRAINT service_listings_service_listings_type_id_fkey
    FOREIGN KEY ("service_listings_type_id") REFERENCES service_listings_type("id")
    ON DELETE RESTRICT ON UPDATE CASCADE;
