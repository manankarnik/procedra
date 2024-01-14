/*
  Warnings:

  - You are about to alter the column `id` on the `User` table. The data in that column will be cast from `BigInt` to `String`. This cast may fail. Please make sure the data in the column can be cast.
  - You are about to alter the column `id` on the `Asset` table. The data in that column will be cast from `BigInt` to `String`. This cast may fail. Please make sure the data in the column can be cast.
  - You are about to alter the column `userId` on the `Asset` table. The data in that column will be cast from `BigInt` to `String`. This cast may fail. Please make sure the data in the column can be cast.
  - Added the required column `thumbnail` to the `Asset` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
CREATE TABLE "_prisma_new_User" (
    "id" STRING NOT NULL,
    "email" STRING NOT NULL,
    "name" STRING NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);
DROP INDEX "User_email_key";
INSERT INTO "_prisma_new_User" ("email","id","name") SELECT "email","id","name" FROM "User";
DROP TABLE "User" CASCADE;
ALTER TABLE "_prisma_new_User" RENAME TO "User";
CREATE UNIQUE INDEX "User_email_key" ON "User"("email");
CREATE TABLE "_prisma_new_Asset" (
    "id" STRING NOT NULL,
    "userId" STRING NOT NULL,
    "title" STRING NOT NULL,
    "description" STRING,
    "public" BOOL NOT NULL,
    "type" STRING NOT NULL,
    "data" JSONB NOT NULL,
    "thumbnail" BYTES NOT NULL,

    CONSTRAINT "Asset_pkey" PRIMARY KEY ("id")
);
INSERT INTO "_prisma_new_Asset" ("data","description","id","public","title","type","userId") SELECT "data","description","id","public","title","type","userId" FROM "Asset";
DROP TABLE "Asset" CASCADE;
ALTER TABLE "_prisma_new_Asset" RENAME TO "Asset";
ALTER TABLE "Asset" ADD CONSTRAINT "Asset_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
