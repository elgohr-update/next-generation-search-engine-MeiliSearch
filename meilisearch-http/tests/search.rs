use std::convert::Into;

use meilisearch_http::routes::search::{SearchQuery, SearchQueryPost};
use assert_json_diff::assert_json_eq;
use serde_json::json;
use serde_json::Value;

mod common;

macro_rules! test_post_get_search {
    ($server:expr, $query:expr, |$response:ident, $status_code:ident | $block:expr) => {
        let post_query: SearchQueryPost = serde_json::from_str(&$query.clone().to_string()).unwrap();
        let get_query: SearchQuery = post_query.into();
        let get_query = ::serde_url_params::to_string(&get_query).unwrap();
        let ($response, $status_code) = $server.search_get(&get_query).await;
        let _ =::std::panic::catch_unwind(|| $block)
            .map_err(|e| panic!("panic in get route: {:?}", e.downcast_ref::<&str>().unwrap()));
        let ($response, $status_code) = $server.search_post($query).await;
        let _ =::std::panic::catch_unwind(|| $block)
            .map_err(|e| panic!("panic in post route: {:?}", e.downcast_ref::<&str>().unwrap()));
    };
}

#[actix_rt::test]
async fn search_with_limit() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json! ({
        "q": "captain",
        "limit": 3
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858
      },
      {
        "id": 271110,
        "popularity": 37.431,
        "vote_average": 7.4,
        "title": "Captain America: Civil War",
        "tagline": "Divided We Fall",
        "overview": "Following the events of Age of Ultron, the collective governments of the world pass an act designed to regulate all superhuman activity. This polarizes opinion amongst the Avengers, causing two factions to side with Iron Man or Captain America, which causes an epic battle between former allies.",
        "director": "Anthony Russo",
        "producer": "Kevin Feige",
        "genres": [
          "Adventure",
          "Action",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/kSBXou5Ac7vEqKd97wotJumyJvU.jpg",
        "vote_count": 15079
      },
      {
        "id": 1771,
        "popularity": 19.657,
        "vote_average": 6.9,
        "title": "Captain America: The First Avenger",
        "tagline": "When patriots become heroes",
        "overview": "During World War II, Steve Rogers is a sickly man from Brooklyn who's transformed into super-soldier Captain America to aid in the war effort. Rogers must stop the Red Skull – Adolf Hitler's ruthless head of weaponry, and the leader of an organization that intends to use a mysterious device of untold powers for world domination.",
        "director": "Joe Johnston",
        "producer": "Kevin Feige", "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/vSNxAJTlD0r02V9sPYpOjqDZXUK.jpg",
        "vote_count": 13853
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_offset() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "Captain",
        "limit": 3,
        "offset": 1
    });

    let expected = json!([
      {
        "id": 271110,
        "popularity": 37.431,
        "vote_average": 7.4,
        "title": "Captain America: Civil War",
        "tagline": "Divided We Fall",
        "overview": "Following the events of Age of Ultron, the collective governments of the world pass an act designed to regulate all superhuman activity. This polarizes opinion amongst the Avengers, causing two factions to side with Iron Man or Captain America, which causes an epic battle between former allies.",
        "director": "Anthony Russo",
        "producer": "Kevin Feige",
        "genres": [
          "Adventure",
          "Action",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/kSBXou5Ac7vEqKd97wotJumyJvU.jpg",
        "vote_count": 15079
      },
      {
        "id": 1771,
        "popularity": 19.657,
        "vote_average": 6.9,
        "title": "Captain America: The First Avenger",
        "tagline": "When patriots become heroes",
        "overview": "During World War II, Steve Rogers is a sickly man from Brooklyn who's transformed into super-soldier Captain America to aid in the war effort. Rogers must stop the Red Skull – Adolf Hitler's ruthless head of weaponry, and the leader of an organization that intends to use a mysterious device of untold powers for world domination.",
        "director": "Joe Johnston",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/vSNxAJTlD0r02V9sPYpOjqDZXUK.jpg",
        "vote_count": 13853
      },
      {
        "id": 268531,
        "popularity": 16.859,
        "vote_average": 6.0,
        "title": "Captain Underpants: The First Epic Movie",
        "tagline": "",
        "overview": "Two mischievous kids hypnotize their mean elementary school principal and turn him into their comic book creation, the kind-hearted and elastic-banded Captain Underpants.",
        "director": "David Soren",
        "producer": "Chris Finnegan",
        "genres": [
          "Action",
          "Animation",
          "Comedy",
          "Family"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AjHZIkzhPXrRNE4VSLVWx6dirK9.jpg",
        "vote_count": 653
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_attribute_to_highlight_wildcard() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "Captain",
        "limit": 1,
        "attributesToHighlight": ["*"]
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_formatted": {
          "id": 299537,
          "popularity": 44.726,
          "vote_average": 7.0,
          "title": "<em>Captain</em> Marvel",
          "tagline": "Higher. Further. Faster.",
          "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, <em>Captain</em> Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
          "director": "Ryan Fleck",
          "producer": "Kevin Feige",
          "genres": [
            "Action",
            "Adventure",
            "Science Fiction"
          ],
          "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
          "vote_count": 7858
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_attribute_to_highlight_1() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToHighlight": ["title"]
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_formatted": {
          "id": 299537,
          "popularity": 44.726,
          "vote_average": 7.0,
          "title": "<em>Captain</em> Marvel",
          "tagline": "Higher. Further. Faster.",
          "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
          "director": "Ryan Fleck",
          "producer": "Kevin Feige",
          "genres": [
            "Action",
            "Adventure",
            "Science Fiction"
          ],
          "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
          "vote_count": 7858
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attribute to highlight title and tagline
// q: Captain
// limit: 1
// attributeToHighlight: title,tagline
#[actix_rt::test]
async fn search_with_attribute_to_highlight_title_tagline() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToHighlight": ["title","tagline"]
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_formatted": {
          "id": 299537,
          "popularity": 44.726,
          "vote_average": 7.0,
          "title": "<em>Captain</em> Marvel",
          "tagline": "Higher. Further. Faster.",
          "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
          "director": "Ryan Fleck",
          "producer": "Kevin Feige",
          "genres": [
            "Action",
            "Adventure",
            "Science Fiction"
          ],
          "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
          "vote_count": 7858
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_attribute_to_highlight_title_overview() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToHighlight": ["title","overview"]
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_formatted": {
          "id": 299537,
          "popularity": 44.726,
          "vote_average": 7.0,
          "title": "<em>Captain</em> Marvel",
          "tagline": "Higher. Further. Faster.",
          "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, <em>Captain</em> Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
          "director": "Ryan Fleck",
          "producer": "Kevin Feige",
          "genres": [
            "Action",
            "Adventure",
            "Science Fiction"
          ],
          "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
          "vote_count": 7858
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_matches() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "Captain",
        "limit": 1,
        "matches": true 
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_matchesInfo": {
          "title": [
            {
              "start": 0,
              "length": 7
            }
          ],
          "overview": [
            {
              "start": 186,
              "length": 7
            }
          ]
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_witch_crop() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "Captain",
        "limit": 1,
        "attributesToCrop": ["overview"],
        "cropLength": 20
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_formatted": {
          "id": 299537,
          "popularity": 44.726,
          "vote_average": 7.0,
          "title": "Captain Marvel",
          "tagline": "Higher. Further. Faster.",
          "overview": ". Set in the 1990s, Captain Marvel is an",
          "director": "Ryan Fleck",
          "producer": "Kevin Feige",
          "genres": [
            "Action",
            "Adventure",
            "Science Fiction"
          ],
          "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
          "vote_count": 7858
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_attributes_to_retrieve() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "Captain",
        "limit": 1,
        "attributesToRetrieve": ["title","tagline","overview","poster_path"],
    });

    let expected = json!([
      {
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg"
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_attributes_to_retrieve_wildcard() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "Captain",
        "limit": 1,
        "attributesToRetrieve": ["*"],
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with filter
// q: Captain
// limit: 3
// filters: director:Anthony%20Russo
#[actix_rt::test]
async fn search_with_filter() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "Captain",
        "limit": 3,
        "filters": "director='Anthony Russo'"
    });

    let expected = json!([
      {
        "id": 271110,
        "popularity": 37.431,
        "vote_average": 7.4,
        "title": "Captain America: Civil War",
        "tagline": "Divided We Fall",
        "overview": "Following the events of Age of Ultron, the collective governments of the world pass an act designed to regulate all superhuman activity. This polarizes opinion amongst the Avengers, causing two factions to side with Iron Man or Captain America, which causes an epic battle between former allies.",
        "director": "Anthony Russo",
        "producer": "Kevin Feige",
        "genres": [
          "Adventure",
          "Action",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/kSBXou5Ac7vEqKd97wotJumyJvU.jpg",
        "vote_count": 15079
      },
      {
        "id": 100402,
        "popularity": 16.418,
        "vote_average": 7.7,
        "title": "Captain America: The Winter Soldier",
        "tagline": "In heroes we trust.",
        "overview": "After the cataclysmic events in New York with The Avengers, Steve Rogers, aka Captain America is living quietly in Washington, D.C. and trying to adjust to the modern world. But when a S.H.I.E.L.D. colleague comes under attack, Steve becomes embroiled in a web of intrigue that threatens to put the world at risk. Joining forces with the Black Widow, Captain America struggles to expose the ever-widening conspiracy while fighting off professional assassins sent to silence him at every turn. When the full scope of the villainous plot is revealed, Captain America and the Black Widow enlist the help of a new ally, the Falcon. However, they soon find themselves up against an unexpected and formidable enemy—the Winter Soldier.",
        "director": "Anthony Russo",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/5TQ6YDmymBpnF005OyoB7ohZps9.jpg",
        "vote_count": 11972
      },
      {
        "id": 299534,
        "popularity": 38.659,
        "vote_average": 8.3,
        "title": "Avengers: Endgame",
        "tagline": "Part of the journey is the end.",
        "overview": "After the devastating events of Avengers: Infinity War, the universe is in ruins due to the efforts of the Mad Titan, Thanos. With the help of remaining allies, the Avengers must assemble once more in order to undo Thanos' actions and restore order to the universe once and for all, no matter what consequences may be in store.",
        "director": "Anthony Russo",
        "producer": "Kevin Feige",
        "genres": [
          "Adventure",
          "Science Fiction",
          "Action"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/or06FN3Dka5tukK1e9sl16pB3iy.jpg",
        "vote_count": 10497
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });

    let expected = json!([
        {
            "id":2770,
            "popularity":17.02,
            "vote_average":6.1,
            "title":"American Pie 2",
            "tagline":"This Summer It's All About Sticking Together.",
            "overview":"The whole gang are back and as close as ever. They decide to get even closer by spending the summer together at a beach house. They decide to hold the biggest party ever to be seen, even if the preparation doesn't always go to plan. Especially when Stifler, Finch and Jim become more close to each other than they ever want to be and when Jim mistakes super glue for lubricant...",
            "director":"J.B. Rogers",
            "producer":"Chris Moore",
            "genres":[
                "Comedy",
                "Romance"
            ],
            "poster_path":"https://image.tmdb.org/t/p/w500/q4LNgUnRfltxzp3gf1MAGiK5LhV.jpg",
            "vote_count":2888
        }
    ]);

    let query = json!({
        "q": "a",
        "limit": 3,
        "filters": "title='american pie 2'"
    });

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });

    let expected = json!([
      {
        "id": 271110,
        "popularity": 37.431,
        "vote_average": 7.4,
        "title": "Captain America: Civil War",
        "tagline": "Divided We Fall",
        "overview": "Following the events of Age of Ultron, the collective governments of the world pass an act designed to regulate all superhuman activity. This polarizes opinion amongst the Avengers, causing two factions to side with Iron Man or Captain America, which causes an epic battle between former allies.",
        "director": "Anthony Russo",
        "producer": "Kevin Feige",
        "genres": [
          "Adventure",
          "Action",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/kSBXou5Ac7vEqKd97wotJumyJvU.jpg",
        "vote_count": 15079
      },
      {
        "id": 100402,
        "popularity": 16.418,
        "vote_average": 7.7,
        "title": "Captain America: The Winter Soldier",
        "tagline": "In heroes we trust.",
        "overview": "After the cataclysmic events in New York with The Avengers, Steve Rogers, aka Captain America is living quietly in Washington, D.C. and trying to adjust to the modern world. But when a S.H.I.E.L.D. colleague comes under attack, Steve becomes embroiled in a web of intrigue that threatens to put the world at risk. Joining forces with the Black Widow, Captain America struggles to expose the ever-widening conspiracy while fighting off professional assassins sent to silence him at every turn. When the full scope of the villainous plot is revealed, Captain America and the Black Widow enlist the help of a new ally, the Falcon. However, they soon find themselves up against an unexpected and formidable enemy—the Winter Soldier.",
        "director": "Anthony Russo",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/5TQ6YDmymBpnF005OyoB7ohZps9.jpg",
        "vote_count": 11972
      }
    ]);
    let query = json!({
        "q": "a",
        "limit": 3,
        "filters": "director='Anthony Russo' AND (title='captain america: civil war' OR title='Captain America: The Winter Soldier')"
    });
    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });

    let expected = json!([
          {
            "id": 299536,
            "popularity": 65.013,
            "vote_average": 8.3,
            "title": "Avengers: Infinity War",
            "tagline": "An entire universe. Once and for all.",
            "overview": "As the Avengers and their allies have continued to protect the world from threats too large for any one hero to handle, a new danger has emerged from the cosmic shadows: Thanos. A despot of intergalactic infamy, his goal is to collect all six Infinity Stones, artifacts of unimaginable power, and use them to inflict his twisted will on all of reality. Everything the Avengers have fought for has led up to this moment - the fate of Earth and existence itself has never been more uncertain.",
            "director": "Anthony Russo",
            "producer": "Kevin Feige",
            "genres": [
              "Adventure",
              "Action",
              "Science Fiction"
            ],
            "poster_path": "https://image.tmdb.org/t/p/w500/7WsyChQLEftFiDOVTGkv3hFpyyt.jpg",
            "vote_count": 16056
          },
          {
            "id": 299534,
            "popularity": 38.659,
            "vote_average": 8.3,
            "title": "Avengers: Endgame",
            "tagline": "Part of the journey is the end.",
            "overview": "After the devastating events of Avengers: Infinity War, the universe is in ruins due to the efforts of the Mad Titan, Thanos. With the help of remaining allies, the Avengers must assemble once more in order to undo Thanos' actions and restore order to the universe once and for all, no matter what consequences may be in store.",
            "director": "Anthony Russo",
            "producer": "Kevin Feige",
            "genres": [
              "Adventure",
              "Science Fiction",
              "Action"
            ],
            "poster_path": "https://image.tmdb.org/t/p/w500/or06FN3Dka5tukK1e9sl16pB3iy.jpg",
            "vote_count": 10497
          },
          {
            "id": 271110,
            "popularity": 37.431,
            "vote_average": 7.4,
            "title": "Captain America: Civil War",
            "tagline": "Divided We Fall",
            "overview": "Following the events of Age of Ultron, the collective governments of the world pass an act designed to regulate all superhuman activity. This polarizes opinion amongst the Avengers, causing two factions to side with Iron Man or Captain America, which causes an epic battle between former allies.",
            "director": "Anthony Russo",
            "producer": "Kevin Feige",
            "genres": [
              "Adventure",
              "Action",
              "Science Fiction"
            ],
            "poster_path": "https://image.tmdb.org/t/p/w500/kSBXou5Ac7vEqKd97wotJumyJvU.jpg",
            "vote_count": 15079
          }
        ]);
    // director = "anthony russo" AND  (title = "captain america: civil war" OR vote_average > 8.0)
    let query = json!({
        "q": "a",
        "limit": 3,
        "filters": "director='anthony russo' AND (title = 'captain america: civil war' OR vote_average > 8.0)"
    });
    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });

    let expected = json!([
          {
            "id": 812,
            "popularity": 36.854,
            "vote_average": 7.6,
            "title": "Aladdin",
            "tagline": "Wish granted!",
            "overview": "Princess Jasmine grows tired of being forced to remain in the palace, so she sneaks out into the marketplace, in disguise, where she meets street-urchin Aladdin.  The couple falls in love, although Jasmine may only marry a prince.  After being thrown in jail, Aladdin becomes embroiled in a plot to find a mysterious lamp, with which the evil Jafar hopes to rule the land.",
            "director": "Ron Clements",
            "producer": "Ron Clements",
            "genres": [
              "Animation",
              "Family",
              "Adventure",
              "Fantasy",
              "Romance"
            ],
            "vote_count": 7156,
            "poster_path": "https://image.tmdb.org/t/p/w500/mjKozYRuHc9j7SmiXmbVmCiAM0A.jpg"
          },
          {
            "id": 348,
            "popularity": 26.175,
            "vote_average": 8.1,
            "title": "Alien",
            "tagline": "In space no one can hear you scream.",
            "overview": "During its return to the earth, commercial spaceship Nostromo intercepts a distress signal from a distant planet. When a three-member team of the crew discovers a chamber containing thousands of eggs on the planet, a creature inside one of the eggs attacks an explorer. The entire crew is unaware of the impending nightmare set to descend upon them when the alien parasite planted inside its unfortunate host is birthed.",
            "director": "Ridley Scott",
            "producer": "David Giler",
            "genres": [
              "Horror",
              "Science Fiction"
            ],
            "vote_count": 8237,
            "poster_path": "https://image.tmdb.org/t/p/w500/2h00HrZs89SL3tXB4nbkiM7BKHs.jpg"
          },
          {
            "id": 73,
            "popularity": 22.887,
            "vote_average": 8.4,
            "title": "American History X",
            "tagline": "Some Legacies Must End.",
            "overview": "Derek Vineyard is paroled after serving 3 years in prison for killing two thugs who tried to break into/steal his truck. Through his brother, Danny Vineyard's narration, we learn that before going to prison, Derek was a skinhead and the leader of a violent white supremacist gang that committed acts of racial crime throughout L.A. and his actions greatly influenced Danny. Reformed and fresh out of prison, Derek severs contact with the gang and becomes determined to keep Danny from going down the same violent path as he did.",
            "director": "Tony Kaye",
            "producer": "John Morrissey",
            "genres": [
              "Drama"
            ],
            "vote_count": 6570,
            "poster_path": "https://image.tmdb.org/t/p/w500/fXepRAYOx1qC3wju7XdDGx60775.jpg"
          }
        ]);

    let query = json!({
        "q": "a",
        "limit": 3,
        "filters": "NOT director = 'anthony russo' AND vote_average > 7.5"
    });

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });

    let expected = json!([]);
    let query = json!({
        "q": "a",
        "filters": "NOT director = 'anthony russo' AND title='Avengers: Endgame'"
    });

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attributes to highlight and matches
// q: Captain
// limit: 1
// attributesToHighlight: [title,overview]
// matches: true
#[actix_rt::test]
async fn search_with_attributes_to_highlight_and_matches() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToHighlight": ["title","overview"],
        "matches": true,
    });

    let expected = json!( [
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_formatted": {
          "id": 299537,
          "popularity": 44.726,
          "vote_average": 7.0,
          "title": "<em>Captain</em> Marvel",
          "tagline": "Higher. Further. Faster.",
          "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, <em>Captain</em> Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
          "director": "Ryan Fleck",
          "producer": "Kevin Feige",
          "genres": [
            "Action",
            "Adventure",
            "Science Fiction"
          ],
          "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
          "vote_count": 7858
        },
        "_matchesInfo": {
          "overview": [
            {
              "start": 186,
              "length": 7
            }
          ],
          "title": [
            {
              "start": 0,
              "length": 7
            }
          ]
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_attributes_to_highlight_and_matches_and_crop() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToCrop": ["overview"],
        "cropLength": 20,
        "attributesToHighlight": ["title","overview"],
        "matches": true
    });

    let expected = json!([
      {
        "id": 299537,
        "popularity": 44.726,
        "vote_average": 7.0,
        "title": "Captain Marvel",
        "tagline": "Higher. Further. Faster.",
        "overview": "The story follows Carol Danvers as she becomes one of the universe’s most powerful heroes when Earth is caught in the middle of a galactic war between two alien races. Set in the 1990s, Captain Marvel is an all-new adventure from a previously unseen period in the history of the Marvel Cinematic Universe.",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "genres": [
          "Action",
          "Adventure",
          "Science Fiction"
        ],
        "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
        "vote_count": 7858,
        "_formatted": {
          "id": 299537,
          "popularity": 44.726,
          "vote_average": 7.0,
          "title": "<em>Captain</em> Marvel",
          "tagline": "Higher. Further. Faster.",
          "overview": ". Set in the 1990s, <em>Captain</em> Marvel is an",
          "director": "Ryan Fleck",
          "producer": "Kevin Feige",
          "genres": [
            "Action",
            "Adventure",
            "Science Fiction"
          ],
          "poster_path": "https://image.tmdb.org/t/p/w500/AtsgWhDnHTq68L0lLsUrCnM7TjG.jpg",
          "vote_count": 7858
        },
        "_matchesInfo": {
          "overview": [
            {
              "start": 20,
              "length": 7
            }
          ],
          "title": [
            {
              "start": 0,
              "length": 7
            }
          ]
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_differents_attributes() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToHighlight": ["title"],
    });

    let expected = json!([
      {
        "title": "Captain Marvel",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "_formatted": {
          "title": "<em>Captain</em> Marvel"
        }
      }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn search_with_differents_attributes_2() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToCrop": ["overview"],
        "cropLength": 10,
    });

    let expected = json!([
      {
      "title": "Captain Marvel",
      "director": "Ryan Fleck",
      "producer": "Kevin Feige",
      "_formatted": {
        "overview": "1990s, Captain Marvel"
      }
    }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attributes to highlight and matches and crop
// q: Captain
// limit: 1
// attributesToRetrieve: [title,producer,director]
// attributesToCrop: [overview:10]
#[actix_rt::test]
async fn search_with_differents_attributes_3() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToCrop": ["overview:10"],
    });

    let expected = json!([
      {
      "title": "Captain Marvel",
      "director": "Ryan Fleck",
      "producer": "Kevin Feige",
      "_formatted": {
        "overview": "1990s, Captain Marvel"
      }
    }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attributes to highlight and matches and crop
// q: Captain
// limit: 1
// attributesToRetrieve: [title,producer,director]
// attributesToCrop: [overview:10,title:0]
#[actix_rt::test]
async fn search_with_differents_attributes_4() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToCrop": ["overview:10","title:0"],
    });

    let expected = json!([
    {
      "title": "Captain Marvel",
      "director": "Ryan Fleck",
      "producer": "Kevin Feige",
      "_formatted": {
        "title": "Captain",
        "overview": "1990s, Captain Marvel"
      }
    }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attributes to highlight and matches and crop
// q: Captain
// limit: 1
// attributesToRetrieve: [title,producer,director]
// attributesToCrop: [*,overview:10]
#[actix_rt::test]
async fn search_with_differents_attributes_5() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToCrop": ["*","overview:10"],
    });

    let expected = json!([
    {
      "title": "Captain Marvel",
      "director": "Ryan Fleck",
      "producer": "Kevin Feige",
      "_formatted": {
        "title": "Captain Marvel",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "overview": "1990s, Captain Marvel"
      }
    }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attributes to highlight and matches and crop
// q: Captain
// limit: 1
// attributesToRetrieve: [title,producer,director]
// attributesToCrop: [*,overview:10]
// attributesToHighlight: [title]
#[actix_rt::test]
async fn search_with_differents_attributes_6() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToCrop": ["*","overview:10"],
        "attributesToHighlight": ["title"],
    });

    let expected = json!([
    {
      "title": "Captain Marvel",
      "director": "Ryan Fleck",
      "producer": "Kevin Feige",
      "_formatted": {
        "title": "<em>Captain</em> Marvel",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "overview": "1990s, Captain Marvel"
      }
    }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attributes to highlight and matches and crop
// q: Captain
// limit: 1
// attributesToRetrieve: [title,producer,director]
// attributesToCrop: [*,overview:10]
// attributesToHighlight: [*]
#[actix_rt::test]
async fn search_with_differents_attributes_7() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToCrop": ["*","overview:10"],
        "attributesToHighlight": ["*"],
    });

    let expected = json!([
    {
      "title": "Captain Marvel",
      "director": "Ryan Fleck",
      "producer": "Kevin Feige",
      "_formatted": {
        "title": "<em>Captain</em> Marvel",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "overview": "1990s, Captain Marvel"
      }
    }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

// Search with attributes to highlight and matches and crop
// q: Captain
// limit: 1
// attributesToRetrieve: [title,producer,director]
// attributesToCrop: [*,overview:10]
// attributesToHighlight: [*,tagline]
#[actix_rt::test]
async fn search_with_differents_attributes_8() {
    let mut server = common::Server::with_uid("movies");
    server.populate_movies().await;

    let query = json!({
        "q": "captain",
        "limit": 1,
        "attributesToRetrieve": ["title","producer","director"],
        "attributesToCrop": ["*","overview:10"],
        "attributesToHighlight": ["*","tagline"],
    });

    let expected = json!([
    {
      "title": "Captain Marvel",
      "director": "Ryan Fleck",
      "producer": "Kevin Feige",
      "_formatted": {
        "title": "<em>Captain</em> Marvel",
        "director": "Ryan Fleck",
        "producer": "Kevin Feige",
        "tagline": "Higher. Further. Faster.",
        "overview": "1990s, Captain Marvel"
      }
    }
    ]);

    test_post_get_search!(server, query, |response, _status_code| {
        assert_json_eq!(expected.clone(), response["hits"].clone(), ordered: false);
    });
}

#[actix_rt::test]
async fn test_faceted_search_valid() {
    let mut server = common::Server::test_server().await;

    // simple tests on attributes with string value
    let body = json!({
        "attributesForFaceting": ["color"]
    });

    server.update_all_settings(body).await;

    let query = json!({
        "q": "a",
        "facetFilters": ["color:green"]
    });

    test_post_get_search!(server, query, |response, _status_code| {
        assert!(!response.get("hits").unwrap().as_array().unwrap().is_empty());
        assert!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .all(|value| value.get("color").unwrap() == "green"));
    });

    let query = json!({
        "q": "a",
        "facetFilters": [["color:blue"]]
    });

    test_post_get_search!(server, query, |response, _status_code| {
        assert!(!response.get("hits").unwrap().as_array().unwrap().is_empty());
        assert!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .all(|value| value.get("color").unwrap() == "blue"));
    });

    let query = json!({
        "q": "a",
        "facetFilters": ["color:Blue"]
    });

    test_post_get_search!(server, query, |response, _status_code| {
        assert!(!response.get("hits").unwrap().as_array().unwrap().is_empty());
        assert!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .all(|value| value.get("color").unwrap() == "blue"));
    });

    // test on arrays: ["tags:bug"]
    let body = json!({
        "attributesForFaceting": ["color", "tags"]
    });

    server.update_all_settings(body).await;

    let query = json!({
        "q": "a",
        "facetFilters": ["tags:bug"]
    });
    test_post_get_search!(server, query, |response, _status_code| {
        assert!(!response.get("hits").unwrap().as_array().unwrap().is_empty());
        assert!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .all(|value| value.get("tags").unwrap().as_array().unwrap().contains(&Value::String("bug".to_owned()))));
    });

    // test and: ["color:blue", "tags:bug"]
    let query = json!({
        "q": "a",
        "facetFilters": ["color:blue", "tags:bug"]
    });
    test_post_get_search!(server, query, |response, _status_code| {
        assert!(!response.get("hits").unwrap().as_array().unwrap().is_empty());
        assert!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .all(|value| value
                .get("color")
                .unwrap() == "blue"
                && value.get("tags").unwrap().as_array().unwrap().contains(&Value::String("bug".to_owned()))));
    });

    // test or: [["color:blue", "color:green"]]
    let query = json!({
        "q": "a",
        "facetFilters": [["color:blue", "color:green"]]
    });
    test_post_get_search!(server, query, |response, _status_code| {
        assert!(!response.get("hits").unwrap().as_array().unwrap().is_empty());
        assert!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .all(|value|
                value
                .get("color")
                .unwrap() == "blue"
                || value
                .get("color")
                .unwrap() == "green"));
    });
    // test and-or: ["tags:bug", ["color:blue", "color:green"]]
    let query = json!({
        "q": "a",
        "facetFilters": ["tags:bug", ["color:blue", "color:green"]]
    });
    test_post_get_search!(server, query, |response, _status_code| {
        assert!(!response.get("hits").unwrap().as_array().unwrap().is_empty());
        assert!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .all(|value|
                value
                .get("tags")
                .unwrap()
                .as_array()
                .unwrap()
                .contains(&Value::String("bug".to_owned()))
                && (value
                    .get("color")
                    .unwrap() == "blue"
                    || value
                    .get("color")
                    .unwrap() == "green")));

    });
}

#[actix_rt::test]
async fn test_faceted_search_invalid() {
    let mut server = common::Server::test_server().await;

    //no faceted attributes set
    let query = json!({
        "q": "a",
        "facetFilters": ["color:blue"]
    });

    test_post_get_search!(server, query, |response, status_code| {

        assert_eq!(status_code, 400);
        assert_eq!(response["errorCode"], "invalid_facet"); 
    }); 

    let body = json!({
        "attributesForFaceting": ["color", "tags"]
    });
    server.update_all_settings(body).await;
    // empty arrays are error
    // []
    let query = json!({
        "q": "a",
        "facetFilters": []
    });

    test_post_get_search!(server, query, |response, status_code| {
        assert_eq!(status_code, 400);
        assert_eq!(response["errorCode"], "invalid_facet"); 
    });
    // [[]]
    let query = json!({
        "q": "a",
        "facetFilters": [[]]
    });

    test_post_get_search!(server, query, |response, status_code| {
        assert_eq!(status_code, 400);
        assert_eq!(response["errorCode"], "invalid_facet"); 
    });

    // ["color:green", []]
    let query = json!({
        "q": "a",
        "facetFilters": ["color:green", []]
    });

    test_post_get_search!(server, query, |response, status_code| {
        assert_eq!(status_code, 400);
        assert_eq!(response["errorCode"], "invalid_facet"); 
    });

    // too much depth
    // [[[]]]
    let query = json!({
        "q": "a",
        "facetFilters": [[[]]]
    });

    test_post_get_search!(server, query, |response, status_code| {
        assert_eq!(status_code, 400);
        assert_eq!(response["errorCode"], "invalid_facet"); 
    });

    // [["color:green", ["color:blue"]]]
    let query = json!({
        "q": "a",
        "facetFilters": [["color:green", ["color:blue"]]]
    });

    test_post_get_search!(server, query, |response, status_code| {
        assert_eq!(status_code, 400);
        assert_eq!(response["errorCode"], "invalid_facet"); 
    });

    // "color:green"
    let query = json!({
        "q": "a",
        "facetFilters": "color:green"
    });

    test_post_get_search!(server, query, |response, status_code| {
        assert_eq!(status_code, 400);
        assert_eq!(response["errorCode"], "invalid_facet"); 
    });
}

#[actix_rt::test]
async fn test_facet_count() {
    let mut server = common::Server::test_server().await;

    // test without facet distribution
    let query = json!({
        "q": "a",
    });
    test_post_get_search!(server, query, |response, _status_code|{
        assert!(response.get("exhaustiveFacetsCount").is_none());
        assert!(response.get("facetsDistribution").is_none());
    });

    // test no facets set, search on color
    let query = json!({
        "q": "a",
        "facetsDistribution": ["color"]
    });
    test_post_get_search!(server, query.clone(), |_response, status_code|{
        assert_eq!(status_code, 400);
    });

    let body = json!({
        "attributesForFaceting": ["color", "tags"]
    });
    server.update_all_settings(body).await;
    // same as before, but now facets are set:
    test_post_get_search!(server, query, |response, _status_code|{
        println!("{}", response);
        assert!(response.get("exhaustiveFacetsCount").is_some());
        assert_eq!(response.get("facetsDistribution").unwrap().as_object().unwrap().values().count(), 1);
    });
    // searching on color and tags
    let query = json!({
        "q": "a",
        "facetsDistribution": ["color", "tags"]
    });
    test_post_get_search!(server, query, |response, _status_code|{
        let facets = response.get("facetsDistribution").unwrap().as_object().unwrap();
        assert_eq!(facets.values().count(), 2);
        assert_ne!(!facets.get("color").unwrap().as_object().unwrap().values().count(), 0);
        assert_ne!(!facets.get("tags").unwrap().as_object().unwrap().values().count(), 0);
    });
    // wildcard
    let query = json!({
        "q": "a",
        "facetsDistribution": ["*"]
    });
    test_post_get_search!(server, query, |response, _status_code|{
        assert_eq!(response.get("facetsDistribution").unwrap().as_object().unwrap().values().count(), 2);
    });
    // wildcard with other attributes:
    let query = json!({
        "q": "a",
        "facetsDistribution": ["color", "*"]
    });
    test_post_get_search!(server, query, |response, _status_code|{
        assert_eq!(response.get("facetsDistribution").unwrap().as_object().unwrap().values().count(), 2);
    });

    // empty facet list
    let query = json!({
        "q": "a",
        "facetsDistribution": []
    });
    test_post_get_search!(server, query, |response, _status_code|{
        assert_eq!(response.get("facetsDistribution").unwrap().as_object().unwrap().values().count(), 0);
    });

    // attr not set as facet passed:
    let query = json!({
        "q": "a",
        "facetsDistribution": ["gender"]
    });
    test_post_get_search!(server, query, |_response, status_code|{
        assert_eq!(status_code, 400);
    });

}

#[actix_rt::test]
#[should_panic]
async fn test_bad_facet_distribution() {
    let mut server = common::Server::test_server().await;
    // string instead of array:
    let query = json!({
        "q": "a",
        "facetsDistribution": "color"
    });
    test_post_get_search!(server, query, |_response, _status_code| {});

    // invalid value in array:
    let query = json!({
        "q": "a",
        "facetsDistribution": ["color", true]
    });
    test_post_get_search!(server, query, |_response, _status_code| {});
}

#[actix_rt::test]
async fn highlight_cropped_text() {
    let mut server = common::Server::with_uid("test");

    let body = json!({
        "uid": "test",
        "primaryKey": "id",
    });
    server.create_index(body).await;

    let doc = json!([
        {
            "id": 1,
            "body": r##"well, it may not work like that, try the following: 
1. insert your trip
2. google your `searchQuery`
3. find a solution 
> say hello"##
        }
    ]);
    server.add_or_replace_multiple_documents(doc).await;

    // tests from #680
    //let query = "q=insert&attributesToHighlight=*&attributesToCrop=body&cropLength=30";
    let query = json!({
        "q": "insert",
        "attributesToHighlight": ["*"],
        "attributesToCrop": ["body"],
        "cropLength": 30,
    });
    let expected_response = "that, try the following: \n1. <em>insert</em> your trip\n2. google your";
    test_post_get_search!(server, query, |response, _status_code|{
        assert_eq!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .get(0)
            .unwrap()
            .as_object()
            .unwrap()
            .get("_formatted")
            .unwrap()
            .as_object()
            .unwrap()
            .get("body")
            .unwrap()
            , &Value::String(expected_response.to_owned()));
    });

    //let query = "q=insert&attributesToHighlight=*&attributesToCrop=body&cropLength=80";
    let query = json!({
        "q": "insert",
        "attributesToHighlight": ["*"],
        "attributesToCrop": ["body"],
        "cropLength": 80,
    });
    let expected_response = "well, it may not work like that, try the following: \n1. <em>insert</em> your trip\n2. google your `searchQuery`\n3. find a solution \n> say hello";
    test_post_get_search!(server, query, |response, _status_code| {
        assert_eq!(response
            .get("hits")
            .unwrap()
            .as_array()
            .unwrap()
            .get(0)
            .unwrap()
            .as_object()
            .unwrap()
            .get("_formatted")
            .unwrap()
            .as_object()
            .unwrap()
            .get("body")
            .unwrap()
            , &Value::String(expected_response.to_owned()));
    });
}

#[actix_rt::test]
async fn well_formated_error_with_bad_request_params() {
    let mut server = common::Server::with_uid("test");
    let query = "foo=bar";
    let (response, _status_code) = server.search_get(query).await;
    assert!(response.get("message").is_some());
    assert!(response.get("errorCode").is_some());
    assert!(response.get("errorType").is_some());
    assert!(response.get("errorLink").is_some());
}
