import prisma from "$lib/prisma";

export async function POST({ request }) {
  const asset = await request.json();
  asset.data = JSON.parse(asset.data);
  asset.thumbnail = Buffer.from(asset.thumbnail);
  await prisma.asset.create({ data: { ...asset } });
}

export async function PUT({ request }) {
  const asset = await request.json();
  asset.data = JSON.parse(asset.data);
  asset.thumbnail = Buffer.from(asset.thumbnail);
  await prisma.asset.update({ where: { id: asset.id }, data: { ...asset } });
}

export async function DELETE({ request }) {
  const asset = await request.json();
  asset.deleted = true;
  await prisma.asset.update({ where: { id: asset.id }, data: { ...asset } });
}
