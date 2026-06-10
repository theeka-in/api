CREATE TABLE listing_categories (
    "id"   UUID PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL UNIQUE,
    "icon" VARCHAR(100)
);
