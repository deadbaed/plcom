use crate::common::wallpapers::WALLPAPERS;
use crate::prelude::*;

pub fn wallpapers_page() -> impl IntoAny {
    let wallpaper_markers = WALLPAPERS
        .iter()
        .map(|wallpaper| {
            format!(
                r#"L.marker({}).addTo(map).bindPopup(`<p>{}<br>{}<br><b>{}</b></p><a href="/?wallpaper={}">Use as wallpaper</a>`);"#,
                wallpaper.gps,
                wallpaper.location.precise,
                wallpaper.location.broad,
                wallpaper.date,
                wallpaper.filename,
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    view! {
        <p class=tw_join!("mb-2")>"Pictures I took around the world"</p>

        <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=" crossorigin=""/>
        <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=" crossorigin=""></script>
        <div id="map" class="w-full h-(--halfscreen)"></div>

        <script inner_html=r#"
            let map = L.map('map').setView([48.858288, 2.294442], 2);
            L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
                maxZoom: 19,
                attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            }).addTo(map);
        "#></script>

        <script inner_html=wallpaper_markers></script>
    }.into_any()
}
