/*
  Warnings:

  - Added the required column `embedding` to the `business_listings` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "listing"."business_listings" ADD COLUMN     "embedding" vector(768) NOT NULL;
