CREATE TYPE media_type AS ENUM ('image', 'video');

CREATE TABLE business_media (
    "id"          UUID NOT NULL,
    "type"        media_type NOT NULL,
    "url"         TEXT NOT NULL,
    "business_id" UUID NOT NULL,
    CONSTRAINT business_media_pkey PRIMARY KEY ("id"),
    CONSTRAINT business_media_business_id_fkey FOREIGN KEY ("business_id") REFERENCES businesses("id") ON DELETE CASCADE ON UPDATE CASCADE
);
