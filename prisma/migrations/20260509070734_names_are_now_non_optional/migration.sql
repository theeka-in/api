/*
  Warnings:

  - You are about to alter the column `name` on the `User` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(50)`.
  - You are about to alter the column `username` on the `User` table. The data in that column could be lost. The data in that column will be cast from `VarChar(60)` to `VarChar(50)`.
  - Made the column `name` on table `User` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "User" ALTER COLUMN "name" SET NOT NULL,
ALTER COLUMN "name" SET DATA TYPE VARCHAR(50),
ALTER COLUMN "username" SET DATA TYPE VARCHAR(50);
