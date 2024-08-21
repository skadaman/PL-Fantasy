# PL-Fantasy
This repo holds a personal project to pull data from the Premier League Fantasy API (draft mode) as a way to practice Rust. The goal is to use the REST API end-points that make the fantasy draft data available tocreate an interface that is more intuitive to use than the webportal. Helpful information on the end points can be found [here](https://www.reddit.com/r/DraftEPL/comments/uw95w0/i_made_end_of_season_infographics_for_my_fpl/). 

For this excercise I pull the league data, all player metadata, week by week formations chosen by the league fantasy mangers, and finally the weekly scores of each player. That data is then collated into objects that can be used to show league standings and week to week scores.  
