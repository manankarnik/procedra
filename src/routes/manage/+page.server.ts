import prisma from "$lib/prisma";

export async function load({ parent }) {
  const { userId, session } = await parent();
  const user = await prisma.user.findUnique({ where: { email: session.user.email } });
  const assets = await prisma.asset.findMany({
    include: { user: true },
    where: { userId: user.id, deleted: false }
  });
  const likes = await prisma.like.findMany({ where: { userId } });
  assets.map((asset) => {
    asset.thumbnail = "data:image/png;base64," + asset.thumbnail.toString("base64");
    asset.likes = likes.filter((like) => like.assetId == asset.id).length;
  });
  return { assets };
}
