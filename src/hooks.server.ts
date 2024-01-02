import { SvelteKitAuth } from "@auth/sveltekit";
import Google from "@auth/sveltekit/providers/google";
import GitHub from "@auth/sveltekit/providers/github";
import Discord from "@auth/sveltekit/providers/discord";
import {
  AUTH_SECRET,
  GOOGLE_ID,
  GOOGLE_SECRET,
  GITHUB_ID,
  GITHUB_SECRET,
  DISCORD_ID,
  DISCORD_SECRET
} from "$env/static/private";

export const handle = SvelteKitAuth({
  secret: AUTH_SECRET,
  trustHost: true,
  providers: [
    Google({
      clientId: GOOGLE_ID,
      clientSecret: GOOGLE_SECRET
    }),
    GitHub({
      clientId: GITHUB_ID,
      clientSecret: GITHUB_SECRET
    }),
    Discord({
      clientId: DISCORD_ID,
      clientSecret: DISCORD_SECRET
    })
  ]
});
