ARG DENO_VERSION=1.34.1

# build wallpapers
FROM denoland/deno:alpine-${DENO_VERSION} as wallpapers

WORKDIR /wallpapers
COPY utils/wallpapers.ts .
COPY public/wallpapers ./wallpapers/
RUN deno run --allow-read --allow-net wallpapers.ts --sourceDir ./wallpapers/ --destinationDir /wallpapers > wallpapers.json

# build astro
FROM node:18.16-alpine as astro

WORKDIR /astro

COPY package.json package-lock.json ./
RUN npm ci

COPY . .
COPY --from=wallpapers /wallpapers/wallpapers.json ./public/
RUN npm run build

# web server
FROM denoland/deno:alpine-${DENO_VERSION}

EXPOSE 8000

WORKDIR /web
COPY --from=astro /astro/dist .
RUN deno cache /web/server/entry.mjs
CMD ["run", "--allow-net", "--allow-read", "--allow-env", "/web/server/entry.mjs"]
