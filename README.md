# scraper
For some reason, I'm running into what I believe is a local network issue. I've reproduced the same problem on several tools (postman, insomnia, hoppscotch) and every GET request to any public website results in an ECONNREFUSED error.

Unfortunately, in the interest of time, I've tabled this solution while I work on the DSL parser. The Typescript code is still uploaded and available to peruse however.

I chose Typescript because it has some guardrails in place vs common Javascript and it's quick to iterate and script something in it.

# parser
I chose to do this in Rust mostly because I wanted to practice around with it some more and it seemed straightforward for a console application. I chose to use a library to do the eval for two reasons: 1) I believe that in a production environment within a small team, we'd likely be using prebuilt libraries where we could and 2) I could have also written it in Python which has an inbuilt eval() function.


### Other Notes
Before my schedules got really wonky, I was working on a Rust project here just to get familiar with it and do some fun stuff: https://github.com/TheAlbertJen/cait-sith 
