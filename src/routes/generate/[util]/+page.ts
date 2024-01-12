import { error } from "@sveltejs/kit";
import init_map from "../../../utils/map";
import init_terrain from "../../../utils/terrain";
import init_planet from "../../../utils/planet";

const utils = { map: init_map, terrain: init_terrain, planet: init_planet };

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
  if (params.util in utils) {
    return { util: params.util, init: utils[params.util] };
  }
  error(404, "Not found");
}
