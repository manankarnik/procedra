import prisma from "$lib/prisma";
import { json } from "@sveltejs/kit";

export async function POST({ request }) {
  const { assetId, userId } = await request.json();
  try {
    await prisma.like.delete({ where: { likeId: { userId, assetId } } });
    return json({ liked: false });
  } catch {
    await prisma.like.create({ data: { userId, assetId } });
    return json({ liked: true });
  }
}
