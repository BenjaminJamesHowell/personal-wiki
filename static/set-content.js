import { getSearchResults } from "./search.js";
import { formatTitle } from "./title.js";
import markdownIt from 'https://cdn.jsdelivr.net/npm/markdown-it@14.1.0/+esm';

const elements = {
	content: document.getElementById("content"),
	title: document.getElementById("title"),
	search: document.getElementById("search"),
	searchResults: document.getElementById("search-results"),
};

export function initSearchBox() {
	elements.search.addEventListener("input", loadSearches);
	loadSearches();

	async function loadSearches() {
		const results = await getSearchResults(elements.search.value.toLowerCase().replaceAll(" ", "_"));
		elements.searchResults.innerHTML = results.map(url =>
			`<a href="/public/${url}">${formatTitle(url)}</a>`
		).slice(0, 5).join("");
	}
}

export function setContent(json) {
	switch (json.requestType) {
		case "PageList": {
			setPageList(json);
			break;
		}

		case "Page": {
			setPage(json);
			break;
		}

		default: {
			throw new Error("Cannot render this request type :/");
		}
	}
}

function setPageList(json) {
	elements.title.innerText = formatTitle(json.title);

	let content = "";
	content += "<ul>";
	for (const page of json.pages) {
		content += `<li><a href="/public/${page}">${formatTitle(page)}</a></li>`;
	}
	content += "</ul>";
	elements.content.innerHTML = content;
}

function setPage(json) {
	elements.title.innerText = formatTitle(json.title);

	const md = markdownIt();
	elements.content.innerHTML = md.render(json.content);
}

