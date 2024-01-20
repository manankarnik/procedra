import prisma from "$lib/prisma";

export async function load({ parent }) {
  const { userId } = await parent();
  const assets = await prisma.asset.findMany({
    include: { user: { select: { id: true, name: true, image: true } } },
    where: { public: true, deleted: false }
  });
  const likes = await prisma.like.findMany();
  assets.map((asset) => {
    asset.thumbnail = "data:image/png;base64," + asset.thumbnail.toString("base64");
    asset.likes = likes.filter((like) => like.assetId == asset.id).length;
    asset.liked = likes.find((like) => like.userId == userId && like.assetId == asset.id)
      ? true
      : false;
  });
  return { assets };
}
