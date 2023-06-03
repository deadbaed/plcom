import { defineConfig } from 'astro/config';
import tailwind from "@astrojs/tailwind";
import compress from "astro-compress";
import deno from "@astrojs/deno";

// https://astro.build/config
export default defineConfig({
  integrations: [tailwind(), compress()],
  compressHTML: true,
  output: "server",
  adapter: deno({port: 8000})
});