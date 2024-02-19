import axios from 'axios';
import { EventEmitter } from 'events';

const linkRegex = new RegExp("(?:https?://)?(?:www\\.).*\\b");
const emitter: EventEmitter = new EventEmitter();

let hostnames = new Map<String, number>();
let linksFound: number = 0;
let linksChecked: number = 0;

emitter.on("parse", function(urlToScrape: String) {
    let url = new URL(urlToScrape.toString());

    try {
        axios.get(url.toString(), {headers: { Accept: "application/json, text/plain, text/html"}}).then(
            (response) => {
                linkRegex.exec(response.data).forEach((linkToValidate: string) => {
                    emitter.emit("validate", linkToValidate);
                    linksFound++;
                });
            }
        ).catch((error) => {
            console.log(error);
        });
    } catch (error) {
        console.log(error);
    }
});

emitter.on("validate", function(url: String) {
    try {
        axios.get(url.toString()).then(() => {
            let toValidate = new URL(url.toString());
            emitter.emit("upsert", toValidate.hostname);
        }).catch((error) => {
            console.log(`Validation error: ${error}`);
        });
    } catch (error) {
        console.log(`Invalid URL found: ${url}`);
    }
    linksChecked++;
    if (linksChecked == linksFound) {
        emitter.emit("complete");
    }
});

emitter.on("upsert", function(hostname: String) {
    if (!hostnames.has(hostname)) {
        hostnames.set(hostname, 1);
    } else {
        hostnames.set(hostname, hostnames.get(hostname) + 1);
    }
});

emitter.on("complete", () => {
    console.log(JSON.stringify(hostnames));
});

let inputUrl = process.argv[process.argv.length - 1].toString();
if (!inputUrl.startsWith("http")) inputUrl = "http://" + inputUrl;
emitter.emit("parse", inputUrl);