CREATE TABLE review_media (
    "id"        UUID NOT NULL,
    "type"      media_type NOT NULL,
    "url"       TEXT NOT NULL,
    "review_id" UUID NOT NULL,
    CONSTRAINT review_media_pkey PRIMARY KEY ("id"),
    CONSTRAINT review_media_review_id_fkey FOREIGN KEY ("review_id") REFERENCES reviews("id") ON DELETE CASCADE ON UPDATE CASCADE
);
