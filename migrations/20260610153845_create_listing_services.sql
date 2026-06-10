CREATE TABLE listing_services (
    "id"              UUID PRIMARY KEY,
    "sub_category_id" UUID NOT NULL,
    "name"            VARCHAR(150) NOT NULL UNIQUE,
    "description"     TEXT,
    "icon"            VARCHAR(100),
    "embedding"       vector(1024),
    CONSTRAINT listing_services_sub_category_id_fkey FOREIGN KEY ("sub_category_id") REFERENCES listing_sub_categories("id") ON DELETE CASCADE
);
