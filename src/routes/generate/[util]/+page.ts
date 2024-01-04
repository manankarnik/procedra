import { error } from "@sveltejs/kit";

const utils = ["texture", "terrain", "planet"];

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
  if (utils.includes(params.util)) {
    return { util: params.util };
  }
  error(404, "Not found");
}
