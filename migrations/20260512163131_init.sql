-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "analytics";

-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "auth";

-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "business";

-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "listing";

-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "review";

-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "shared";

-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "users";

-- CreateEnum
CREATE TYPE "business"."day_of_week" AS ENUM ('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday');

-- CreateEnum
CREATE TYPE "business"."business_hour_type" AS ENUM ('closed', 'open_24_hours', 'custom_range');

-- CreateEnum
CREATE TYPE "shared"."media_type" AS ENUM ('image', 'video');

-- CreateTable
CREATE TABLE "analytics"."views" (
    "id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id" UUID NOT NULL,
    "business_id" UUID NOT NULL,
    "listing_id" UUID NOT NULL,

    CONSTRAINT "views_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "auth"."accounts" (
    "id" UUID NOT NULL,
    "phone" BIGINT NOT NULL,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "accounts_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "auth"."sessions" (
    "token" TEXT NOT NULL,
    "user_agent" TEXT NOT NULL,
    "ip_address" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "account_id" UUID NOT NULL,

    CONSTRAINT "sessions_pkey" PRIMARY KEY ("token")
);

-- CreateTable
CREATE TABLE "business"."businesses" (
    "id" UUID NOT NULL,
    "phone_number" BIGINT NOT NULL,
    "is_closed" BOOLEAN NOT NULL DEFAULT false,
    "title" TEXT NOT NULL,
    "logo" TEXT,
    "description" TEXT,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "owner_id" UUID NOT NULL,

    CONSTRAINT "businesses_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "business"."business_addresses" (
    "complete_address" TEXT NOT NULL,
    "city" TEXT NOT NULL,
    "state" TEXT NOT NULL,
    "pincode" INTEGER NOT NULL,
    "latitude" DOUBLE PRECISION NOT NULL,
    "longitude" DOUBLE PRECISION NOT NULL,
    "radius" DOUBLE PRECISION NOT NULL,
    "business_id" UUID NOT NULL,

    CONSTRAINT "business_addresses_pkey" PRIMARY KEY ("business_id")
);

-- CreateTable
CREATE TABLE "business"."business_hours" (
    "id" UUID NOT NULL,
    "day" "business"."day_of_week" NOT NULL,
    "hours_type" "business"."business_hour_type" NOT NULL,
    "open_time" TIME(6),
    "close_time" TIME(6),
    "business_id" UUID NOT NULL,

    CONSTRAINT "business_hours_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "business"."business_media" (
    "id" UUID NOT NULL,
    "type" "shared"."media_type" NOT NULL,
    "url" TEXT NOT NULL,
    "business_id" UUID NOT NULL,

    CONSTRAINT "business_media_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "listing"."business_listings" (
    "id" UUID NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "logo" TEXT,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "business_id" UUID NOT NULL,
    "product_listing_id" UUID,
    "service_listing_id" UUID,

    CONSTRAINT "business_listings_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "listing"."product_listings" (
    "id" UUID NOT NULL,
    "price" DOUBLE PRECISION NOT NULL,
    "stock" INTEGER NOT NULL,

    CONSTRAINT "product_listings_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "listing"."service_listings" (
    "id" UUID NOT NULL,
    "price" TEXT NOT NULL,
    "available" BOOLEAN NOT NULL,

    CONSTRAINT "service_listings_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "listing"."listing_categories" (
    "id" UUID NOT NULL,
    "value" TEXT NOT NULL,
    "listing_id" UUID NOT NULL,

    CONSTRAINT "listing_categories_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "listing"."listing_tags" (
    "id" UUID NOT NULL,
    "value" TEXT NOT NULL,
    "listing_id" UUID NOT NULL,

    CONSTRAINT "listing_tags_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "listing"."listing_product_specs" (
    "id" UUID NOT NULL,
    "group_name" TEXT NOT NULL,
    "key" TEXT NOT NULL,
    "value" TEXT NOT NULL,
    "product_listing_id" UUID NOT NULL,

    CONSTRAINT "listing_product_specs_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "listing"."listing_media" (
    "id" UUID NOT NULL,
    "type" "shared"."media_type" NOT NULL,
    "url" TEXT NOT NULL,
    "listing_id" UUID NOT NULL,

    CONSTRAINT "listing_media_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "review"."reviews" (
    "id" UUID NOT NULL,
    "rating" INTEGER NOT NULL,
    "title" TEXT NOT NULL,
    "comment" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id" UUID NOT NULL,
    "business_id" UUID NOT NULL,
    "listing_id" UUID NOT NULL,

    CONSTRAINT "reviews_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "review"."review_media" (
    "id" UUID NOT NULL,
    "type" "shared"."media_type" NOT NULL,
    "url" TEXT NOT NULL,
    "review_id" UUID NOT NULL,

    CONSTRAINT "review_media_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "users"."users" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "avatar" TEXT,
    "account_id" UUID NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "users"."user_addresses" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "complete_address" TEXT NOT NULL,
    "city" TEXT NOT NULL,
    "state" TEXT NOT NULL,
    "pincode" INTEGER NOT NULL,
    "latitude" DOUBLE PRECISION NOT NULL,
    "longitude" DOUBLE PRECISION NOT NULL,
    "user_id" UUID NOT NULL,

    CONSTRAINT "user_addresses_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "views_user_id_listing_id_key" ON "analytics"."views"("user_id", "listing_id");

-- CreateIndex
CREATE UNIQUE INDEX "accounts_phone_key" ON "auth"."accounts"("phone");

-- CreateIndex
CREATE UNIQUE INDEX "business_hours_business_id_day_key" ON "business"."business_hours"("business_id", "day");

-- CreateIndex
CREATE UNIQUE INDEX "business_listings_product_listing_id_key" ON "listing"."business_listings"("product_listing_id");

-- CreateIndex
CREATE UNIQUE INDEX "business_listings_service_listing_id_key" ON "listing"."business_listings"("service_listing_id");

-- CreateIndex
CREATE UNIQUE INDEX "reviews_user_id_listing_id_key" ON "review"."reviews"("user_id", "listing_id");

-- CreateIndex
CREATE UNIQUE INDEX "users_account_id_key" ON "users"."users"("account_id");

-- AddForeignKey
ALTER TABLE "analytics"."views" ADD CONSTRAINT "views_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"."users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "analytics"."views" ADD CONSTRAINT "views_business_id_fkey" FOREIGN KEY ("business_id") REFERENCES "business"."businesses"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "analytics"."views" ADD CONSTRAINT "views_listing_id_fkey" FOREIGN KEY ("listing_id") REFERENCES "listing"."business_listings"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "auth"."sessions" ADD CONSTRAINT "sessions_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "auth"."accounts"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "business"."businesses" ADD CONSTRAINT "businesses_owner_id_fkey" FOREIGN KEY ("owner_id") REFERENCES "users"."users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "business"."business_addresses" ADD CONSTRAINT "business_addresses_business_id_fkey" FOREIGN KEY ("business_id") REFERENCES "business"."businesses"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "business"."business_hours" ADD CONSTRAINT "business_hours_business_id_fkey" FOREIGN KEY ("business_id") REFERENCES "business"."businesses"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "business"."business_media" ADD CONSTRAINT "business_media_business_id_fkey" FOREIGN KEY ("business_id") REFERENCES "business"."businesses"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "listing"."business_listings" ADD CONSTRAINT "business_listings_business_id_fkey" FOREIGN KEY ("business_id") REFERENCES "business"."businesses"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "listing"."business_listings" ADD CONSTRAINT "business_listings_product_listing_id_fkey" FOREIGN KEY ("product_listing_id") REFERENCES "listing"."product_listings"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "listing"."business_listings" ADD CONSTRAINT "business_listings_service_listing_id_fkey" FOREIGN KEY ("service_listing_id") REFERENCES "listing"."service_listings"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "listing"."listing_categories" ADD CONSTRAINT "listing_categories_listing_id_fkey" FOREIGN KEY ("listing_id") REFERENCES "listing"."business_listings"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "listing"."listing_tags" ADD CONSTRAINT "listing_tags_listing_id_fkey" FOREIGN KEY ("listing_id") REFERENCES "listing"."business_listings"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "listing"."listing_product_specs" ADD CONSTRAINT "listing_product_specs_product_listing_id_fkey" FOREIGN KEY ("product_listing_id") REFERENCES "listing"."product_listings"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "listing"."listing_media" ADD CONSTRAINT "listing_media_listing_id_fkey" FOREIGN KEY ("listing_id") REFERENCES "listing"."business_listings"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "review"."reviews" ADD CONSTRAINT "reviews_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"."users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "review"."reviews" ADD CONSTRAINT "reviews_business_id_fkey" FOREIGN KEY ("business_id") REFERENCES "business"."businesses"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "review"."reviews" ADD CONSTRAINT "reviews_listing_id_fkey" FOREIGN KEY ("listing_id") REFERENCES "listing"."business_listings"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "review"."review_media" ADD CONSTRAINT "review_media_review_id_fkey" FOREIGN KEY ("review_id") REFERENCES "review"."reviews"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "users"."users" ADD CONSTRAINT "users_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "auth"."accounts"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "users"."user_addresses" ADD CONSTRAINT "user_addresses_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"."users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
