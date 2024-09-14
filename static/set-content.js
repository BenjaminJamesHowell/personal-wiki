import { getSearchResults } from "./search.js";
import { formatTitle } from "./title.js";
import markdownIt from "https://cdn.jsdelivr.net/npm/markdown-it@14.1.0/+esm";
import yaml from "https://cdn.jsdelivr.net/npm/yaml@2.5.1/+esm";
import markdownItFrontMatter from "https://cdn.jsdelivr.net/npm/markdown-it-front-matter@0.2.4/+esm";

const elements = {
	content: document.getElementById("content"),
	title: document.getElementById("title"),
	search: document.getElementById("search"),
	searchResults: document.getElementById("search-results"),
	categories: document.getElementById("categories"),
	infoBox: document.getElementById("info-box"),
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

		case "Category": {
			setCategory(json);
			break;
		}

		default: {
			throw new Error("Cannot render this request type :/");
		}
	}
}

function setCategory(json) {
	elements.title.innerText = formatTitle(json.title);

	let content = "";
	content += "<ul>";
	for (const page of json.pages) {
		content += `<li><a href="/public/${page}">${formatTitle(page)}</a></li>`;
	}
	content += "</ul>";
	elements.content.innerHTML = content;
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

	const md = markdownIt()
		.use(markdownItFrontMatter, fm => setPageFrontMatter(json, fm));
	elements.content.innerHTML = md.render(json.content);
}

function setPageFrontMatter(page, fmStr) {
	const fmYaml = yaml.parse(fmStr);
	const fm = {
		categories: [],
		infoBox: [],
		...fmYaml,
	};
	console.log(fm);

	// Categories
	elements.categories.innerHTML = "";
	for (const category of fm.categories) {
		elements.categories.innerHTML += `<a href="/public/category/${category}">${formatTitle(category)}</a>`;
	}

	// Info Box
	elements.infoBox.innerHTML = `<h2 class="title">${formatTitle(page.title)}</h2>`;
	if (fm.infoBox.length > 0) {
		elements.infoBox.style.display = "block";
	}

	for (const { name, value } of fm.infoBox) {
		elements.infoBox.innerHTML += `
<div class="property">
	<div class="property-name">
		${name}
	</div>

	<div class="property-value">
		${value}
	</div>
</div>
`;
	}
}

