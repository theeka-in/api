/*
  Warnings:

  - Added the required column `cat2` to the `User` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "User" ADD COLUMN     "cat2" INTEGER NOT NULL;
