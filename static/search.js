import { getPage } from "./api.js";

let pagesCache = undefined;

export async function getSearchResults(filter) {
	if (pagesCache === undefined) {
		pagesCache = await getPage("/");
	}

	const { pages } = pagesCache;

	return pages.filter(page => {
		return page.includes(filter);
	});
}
