# scraper
For some reason, I'm running into what I believe is a local network issue. I've reproduced the same problem on several tools (postman, insomnia, hoppscotch) and every GET request to any public website results in an ECONNREFUSED error.

Unfortunately, in the interest of time, I've tabled this solution while I work on the DSL parser. The Typescript code is still uploaded and available to peruse however.

I chose Typescript because it has some guardrails in place vs common Javascript and it's quick to iterate and script something in it.

# parser
I chose to do this in Rust mostly because I wanted to practice around with it some more and it seemed straightforward for a console application. I chose to use a library to do the eval for two reasons: 1) I believe that in a production environment within a small team, we'd likely be using prebuilt libraries where we could and 2) I could have also written it in Python which has an inbuilt eval() function.

Addendum: I also want to point out that I recall doing a problem similar to this in school awhile ago and the general gist was to read the string in chunks and push/pop items onto stacks and evaluate with respect to mathematical order of operations. I think that probably could have worked here, with some tweaking to account for the comparators, but I also felt that using a third party library was the correct choice in terms of productivity and time savings.


### Other Notes
Before my schedules got really wonky, I was working on a Rust project here just to get familiar with it and do some fun stuff: https://github.com/TheAlbertJen/cait-sith 
