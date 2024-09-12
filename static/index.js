import { getPage } from "./api.js";
import { parseUrl } from "./parse-url.js";
import { initSearchBox, setContent } from "./set-content.js";

async function main() {
	const url = parseUrl(location.href);
	const json = await getPage(url);
	setContent(json);
	initSearchBox();
}

main();

