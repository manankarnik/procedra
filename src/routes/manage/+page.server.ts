import prisma from "$lib/prisma";

export async function load({ parent, locals }) {
  const data = await parent();
  const user = await prisma.user.findUnique({ where: { email: data.session.user.email } });
  const assets = await prisma.asset.findMany({ include: { user: true }, where: { userId: user.id, deleted: false } });
  assets.map(
    (asset) => (asset.thumbnail = "data:image/png;base64," + asset.thumbnail.toString("base64"))
  );
  return { assets };
}
