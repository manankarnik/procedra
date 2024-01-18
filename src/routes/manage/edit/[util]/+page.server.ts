import { error, redirect } from "@sveltejs/kit";
import prisma from "$lib/prisma";

const utils = ["map", "terrain", "planet"];
/** @type {import('./$types').PageLoad} */
export async function load({ parent, params, url }) {
  const { userId } = await parent();
  if (!utils.includes(params.util)) error(404, "Not found");
  const id = url.searchParams.get("id");
  if (id) {
    const asset = await prisma.asset.findUnique({ where: { id } });
    if (asset?.userId != userId) redirect(302, "/");
    return {
      asset: {
        ...asset,
        thumbnail: "data:image/png;base64," + asset.thumbnail.toString("base64"),
        data: JSON.stringify(asset.data)
      },
      util: params.util
    };
  }
}
