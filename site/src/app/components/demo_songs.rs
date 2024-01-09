use super::*;

mod song_bar;
mod song_name;

use song_bar::SongBar;
use song_name::SongName;

#[component]
pub fn DemoSongs() -> impl IntoView {
    let songs = vec![
        ("All About That Base", ""),
        ("Are You Old Enough", ""),
        ("Blue Rose Is", ""),
        ("Blue Suade Shoes", ""),
        ("Could I Have This Dance", ""),
        ("Dear Future Husband", ""),
        ("Ex's and Oh's", ""),
        ("Geronimo", ""),
        ("Happy", ""),
        ("Help Me Rhonda", ""),
        ("Horses", ""),
        ("Little Lies", ""),
        ("Rose Garden", ""),
        ("Somebody Like You", ""),
        ("Wagon Wheel", ""),
    ];

    view! {
        <div class="page-block">
            <h2 class="text-2xl">"Demo Songs"</h2>

            <div class="flex flex-row mt-2">
                <div class="flex flex-col pr-4">
                    {songs.iter()
                        .map(|(name, _)| view! { <SongName name={*name} /> })
                        .collect::<Vec<_>>()}
                </div>

                <div class="flex flex-col flex-grow">
                    {songs.into_iter()
                        .map(|(_, href)| view! { <SongBar _href={href} /> })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
