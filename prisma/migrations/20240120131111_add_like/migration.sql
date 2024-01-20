-- CreateTable
CREATE TABLE "Like" (
    "userId" STRING NOT NULL,
    "assetId" STRING NOT NULL,

    CONSTRAINT "Like_pkey" PRIMARY KEY ("userId","assetId")
);

-- AddForeignKey
ALTER TABLE "Like" ADD CONSTRAINT "Like_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Like" ADD CONSTRAINT "Like_assetId_fkey" FOREIGN KEY ("assetId") REFERENCES "Asset"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
