-- AlterTable
ALTER TABLE "Asset" ADD COLUMN     "deleted" BOOL NOT NULL DEFAULT false;

-- AlterTable
ALTER TABLE "User" ADD COLUMN     "image" STRING;
