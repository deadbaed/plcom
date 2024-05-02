use crate::prelude::*;

#[component]
pub fn WallpapersPage() -> impl IntoView {
    use leptos_leaflet::{TileLayer, MapContainer, Position, Marker, position, Popup};

    let wallpapers = WALLPAPERS;

    view! {
        <ContentPage title="Wallpapers">
            <Stylesheet href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"/>
            <Script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"/>
            <p class=tw_join!("mb-2")>"Pictures I took around the world"</p>
            <MapContainer
                center=Position::new(0.0, 0.0)
                zoom=1.0
                class=tw_join!("w-full", "h-halfscreen")
            >
                <TileLayer
                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                />
                <For
                    each=move || wallpapers
                    key=|w| w.filename
                    children=move |w: Wallpaper| {
                        let uri = format!("/wallpapers/{}", w.filename);
                        view! {
                            <Marker position=position!(
                                w.gps.latitude.into(), w.gps.longitude.into()
                            )>
                                <Popup>
                                    <a target="_blank" href=uri>
                                        {w.location.precise}
                                    </a>
                                    <br/>
                                    {w.location.broad}
                                    <br/>
                                    <b>{w.date}</b>
                                </Popup>
                            </Marker>
                        }
                    }
                />

            </MapContainer>
        </ContentPage>
    }
}
