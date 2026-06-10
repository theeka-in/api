CREATE TABLE listing_sub_categories (
    "id"          UUID PRIMARY KEY,
    "category_id" UUID NOT NULL,
    "name"        VARCHAR(100) NOT NULL UNIQUE,
    "icon"        VARCHAR(100),
    CONSTRAINT listing_sub_categories_category_id_fkey FOREIGN KEY ("category_id") REFERENCES listing_categories("id") ON DELETE CASCADE
);
