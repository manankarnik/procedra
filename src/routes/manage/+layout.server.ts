import { redirect } from "@sveltejs/kit";

export async function load({ parent }) {
  const { userId, session } = await parent();
  if (!session) redirect(302, "/");
  return { userId, session };
}
