use super::*;

mod andrew;
mod erin;

use andrew::Andrew;
use erin::Erin;

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
        <div class="page-block flex flex-col">
            <h1 class="text-2xl">"About Us"</h1>

            <Erin />
            <Andrew />

            <p class="mb-2">"Overime Duo can keep it country, rock along with the 50’s and 60’s rock 'n' rollers, or keep you singing and dancing along with songs from our 70’s and 80’s list and beyond."</p>
            <p class="mb-2">"There's only one thing you need to do: \"Get on the dance floor and dance the night away OR sit back and relax and listen\"."</p>

            <p class="mb-2 font-bold">"Some of the artists we cover:"</p>
            <p class="mb-2">"Van Morrison, Racey, Dion, Creedence, Tina Turner, Abba, Ellie King, Ol' 55, The Dixie Chicks, Dire Straits,  Cold Chisel, Brooks & Dunn, Pharrell Williams, John Cougar, Screaming Jets, Billy Joe Spears, Shania Twain, Elvis, Bob Segar, Bill Haley, Suzi Quatro, Dragon, Ed Sheeran, Lee Kernaghan, Boney M, ELO, Tones & I, Fleetwood Mac, INXS, Dwight Yoakam, Walk The Moon, Smokie, The Mavericks, Roy Orbison, Eddie Rabbit, Tina Arena,  Seudo Echo, Queen, Meghan Trainor, John Fogerty, The Cars,  Mental As Anything,  Kenny Rogers,  Bon Jovi, John Paul Young,  Sheppard, Bryan Adams, Shakin' Stevens, Duran Duran, Coldplay, Keith Urban, The Divinyls, Bruce Springsteen, Sweet, The Monkeys, Blues Brothers, The Choirboys, Kim Wilde, Willie Nelson,  George Ezra, Daddy Cool, Anne Murray, Cliff Richard, Pointer Sisters, Daryl Braithwaite, Darius Rucker, Kid Rock, The Commitments, Lou Bega, Boz Scaggs, Lynn Anderson, Paul Norton.......... Just to name a few."</p>

            <p class="mb-2 font-bold">"Some of the songs we perform:"</p>
            <p class="mb-2">"Geronimo, Footloose, Betty Davis Eyes, April Sun In Cuba, On the Prowl, Man I feel like a woman, Summer Of 69, Perfect, Don’t Change, The Wanderer, Dance Monkey, Boys In Boots, Happy,  Jailhouse Rock, All About the Bass, R & R Girls, Evie, Shut up & dance, Green Door, Some Girls Do, Funky Town,  Eagle Rock, Rock Around The Clock, Brown Eyed Girl, Oh Carol, The Nutbush,   R & R Waltz, Dose Your Mother Know, Black Coffee, Boys In Town, Echo Beach, Lets Go, Neon Moon,  YMCA, The Outback Club, Dancing Queen, Little Lies, Somebody Like You, The Gambler,  I'm Outa Here,  Stuck In The Middle,  Pizzarico, To Many Times,  Boot Scootin’ Boogie, Love Is In The Air, Don't Stop, R & R is King, I'm A Believer, Bad Moon Rising, Drop The Pilot, The Horses, I Love A Rainy Night, Rockabilly Jive, Dear Future Husband, Hurts So Good,  Gold On the Ceiling, Devil Gate Drive, Help Me Rhonda, Heaven Help My Heart, Wagon Wheel, Fox On The Run.......... Just to name a few."</p>
        </div>
    }
}
