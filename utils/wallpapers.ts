
import exifr from "npm:exifr@^7.1.3";
import { mime } from "https://deno.land/x/mimetypes@v1.0.0/mod.ts";
import { parse } from "https://deno.land/std@0.190.0/flags/mod.ts";

interface Arguments {
    sourceDir?: string;
    destinationDir?: string;
}

const cli: Arguments = parse(Deno.args, {
    string: ["sourceDir", "destinationDir"],
});

if (cli.sourceDir === undefined) {
    console.error("Need source folder for images (relative directory where deno script is ran)");
    Deno.exit(1);
}

if (cli.destinationDir === undefined) {
    console.error("Need destination folder for images");
    Deno.exit(1);
}

// Put all wallpapers here
const imagesPath = `${Deno.cwd()}/${cli.sourceDir}`;

interface MyWallpaper {
    file: string;
    location: string;
    date: string;
}

const wallpapers: MyWallpaper[] = [];

// For each file in the wallpapers directory
for await (const dirEntry of Deno.readDir(imagesPath)) {

    // Make sure it is a file and it is an image
    if (!dirEntry.isFile || !mime.getType(dirEntry.name)?.startsWith("image/")) {
        continue;
    }

    // Don't pollute stdout, that's where the json will be put
    console.error(`Processing ${dirEntry.name}`);

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
    // Documentation: https://nominatim.org/release-docs/develop/api/Reverse/
    const response = await fetch(
        `https://nominatim.openstreetmap.org/reverse?format=json&lat=${exif.latitude}&lon=${exif.longitude}&zoom=8`,
        { headers: { "User-Agent": "https://philippeloctaux.com" } }
    );
    const data = await response.json();
    const location = data?.display_name;

    // Final filename
    const file = `${cli.destinationDir}/${dirEntry.name}`;

    wallpapers.push({
        file,
        date,
        location,
    });

}

// Put final array to stdout
console.log(JSON.stringify(wallpapers));