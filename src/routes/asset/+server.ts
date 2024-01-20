import prisma from "$lib/prisma";
import { json } from "@sveltejs/kit";

export async function POST({ request }) {
  const asset = await request.json();
  asset.data = JSON.parse(asset.data);
  asset.thumbnail = Buffer.from(asset.thumbnail);
  await prisma.asset.create({ data: { ...asset } });
  return json({ success: true });
}

export async function PUT({ request }) {
  const asset = await request.json();
  asset.data = JSON.parse(asset.data);
  asset.thumbnail = Buffer.from(asset.thumbnail);
  await prisma.asset.update({ where: { id: asset.id }, data: { ...asset } });
  return json({ success: true });
}

export async function DELETE({ request }) {
  const asset = await request.json();
  asset.deleted = true;
  await prisma.asset.update({ where: { id: asset.id }, data: { ...asset } });
  return json({ success: true });
}
