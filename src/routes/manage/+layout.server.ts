import { redirect } from "@sveltejs/kit";

export async function load({ locals }) {
  const session = await locals.getSession();
  if (!session) redirect(302, "/");
  return { session };
}
