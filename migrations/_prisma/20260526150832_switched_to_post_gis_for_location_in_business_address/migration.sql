/*
  Warnings:

  - You are about to drop the column `complete_address` on the `business_addresses` table. All the data in the column will be lost.
  - You are about to drop the column `latitude` on the `business_addresses` table. All the data in the column will be lost.
  - You are about to drop the column `longitude` on the `business_addresses` table. All the data in the column will be lost.
  - Added the required column `address_line1` to the `business_addresses` table without a default value. This is not possible if the table is not empty.
  - Added the required column `address_line2` to the `business_addresses` table without a default value. This is not possible if the table is not empty.
  - Added the required column `location` to the `business_addresses` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "business"."business_addresses" DROP COLUMN "complete_address",
DROP COLUMN "latitude",
DROP COLUMN "longitude",
ADD COLUMN     "address_line1" TEXT NOT NULL,
ADD COLUMN     "address_line2" TEXT NOT NULL,
ADD COLUMN     "landmark" TEXT,
ADD COLUMN     "location" geometry(Point, 4326) NOT NULL,
ALTER COLUMN "pincode" SET DATA TYPE TEXT;
