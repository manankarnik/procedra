/*
  Warnings:

  - You are about to drop the column `private` on the `Asset` table. All the data in the column will be lost.
  - Added the required column `public` to the `Asset` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Asset" DROP COLUMN "private";
ALTER TABLE "Asset" ADD COLUMN     "public" BOOL NOT NULL;
