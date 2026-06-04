TRUNCATE TABLE "listing"."business_listings" CASCADE;

ALTER TABLE "listing"."business_listings"
ALTER COLUMN "embedding" TYPE vector(1024);