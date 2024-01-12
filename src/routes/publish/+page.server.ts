import prisma from "$lib/prisma";

/** @type {import('./$types').Actions} */
export const actions = {
  default: async ({ request, locals }) => {
    const asset = await request.json();
    const session = await locals.getSession();
    const user = await prisma.user.findUnique({
      where: { email: session.user.email }
    });
    asset.userId = user?.id;
    const result = await prisma.asset.create({ data: { ...asset } });
    console.log(result);
  }
};
