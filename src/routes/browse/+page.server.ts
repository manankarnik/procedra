import prisma from "$lib/prisma";

export async function load({ params }) {
  const assets = await prisma.asset.findMany({ where: { public: true } });
  assets.map(
    (asset) => (asset.thumbnail = "data:image/png;base64," + asset.thumbnail.toString("base64"))
  );
  return { assets };
}
