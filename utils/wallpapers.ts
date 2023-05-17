
import exifr from "npm:exifr@^7.1.3";
import { writeJson } from "https://deno.land/x/jsonfile@1.0.0/mod.ts";

const publicPath = "/public";
const wallpapersPath = "/wallpapers";
const imagesPath = `${Deno.cwd()}${publicPath}${wallpapersPath}`;

interface MyWallpaper {
    file: string;
    location: string;
    date: string;
}

const wallpapers: MyWallpaper[] = [];

// For each file in the wallpapers directory
for await (const dirEntry of Deno.readDir(imagesPath)) {

    if (!dirEntry.isFile || dirEntry.name === ".gitkeep") {
        continue;
    }

    // Open file
    const path = `${imagesPath}/${dirEntry.name}`;
    const bytes = await Deno.readFile(path);

    // Parse exif
    const exif = await exifr.parse(bytes);

    // Timezones are too hard
    const timestamp = Date.parse(exif.DateTimeOriginal);
    const dateObject = new Date(timestamp);
    const date = dateObject.toISOString().split("T")[0];

    // Http request to get location
    const response = await fetch(
        `https://nominatim.openstreetmap.org/reverse?format=json&lat=${exif.latitude}&lon=${exif.longitude}&zoom=5`,
        { headers: { "User-Agent": "https://philippeloctaux.com" } }
    );
    const data = await response.json();
    const location = data?.display_name;

    // Final filename
    const file = `${wallpapersPath}/${dirEntry.name}`;

    console.log(`Processed ${file}`);

    wallpapers.push({
        file,
        date,
        location,
    });

}

// write to /public/wallpapers.json
const jsonPath = `${Deno.cwd()}${publicPath}/wallpapers.json`;
await writeJson(jsonPath, wallpapers);
console.log(`Wrote to ${jsonPath}`);
