import prisma from "$lib/prisma";
import { json } from "@sveltejs/kit";

export async function POST({ event, request }) {
  const { tier, color, userId } = await request.json();
  await prisma.user.update({ where: { id: userId }, data: { [tier]: color } });
  return json({ success: true });
}
