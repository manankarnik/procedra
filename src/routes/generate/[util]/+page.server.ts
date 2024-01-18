import { error } from "@sveltejs/kit";
import prisma from "$lib/prisma";

const utils = ["map", "terrain", "planet"];
/** @type {import('./$types').PageLoad} */
export async function load({ params, url }) {

  if (utils.includes(params.util)) {
    let asset;
    if (url.searchParams.get("id")) {
      if (url.searchParams.get("id")) {
        asset = await prisma.asset.findUnique({ where: { id: url.searchParams.get("id") } });
      }
    }
    return { asset: { data: JSON.stringify(asset?.data) }, util: params.util };
  }
  error(404, "Not found");
}
