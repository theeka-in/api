/*
  Warnings:

  - You are about to drop the column `cat` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `cat2` on the `User` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "User" DROP COLUMN "cat",
DROP COLUMN "cat2";
