CREATE TYPE day_of_week AS ENUM ('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday');
CREATE TYPE business_hour_type AS ENUM ('closed', 'open_24_hours', 'custom_range');

CREATE TABLE business_hours (
    "id"          UUID NOT NULL,
    "day"         day_of_week NOT NULL,
    "hours_type"  business_hour_type NOT NULL,
    "open_time"   TIME(6),
    "close_time"  TIME(6),
    "business_id" UUID NOT NULL,
    CONSTRAINT business_hours_pkey PRIMARY KEY ("id"),
    CONSTRAINT business_hours_business_id_fkey FOREIGN KEY ("business_id") REFERENCES businesses("id") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX business_hours_business_id_day_key ON business_hours("business_id", "day");
