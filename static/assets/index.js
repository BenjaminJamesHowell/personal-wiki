const elements = {
	searchBox: document.getElementById("search-box"),
	searchResults: document.getElementById("search-results"),
};

function formatTitle(internal) {
	const first = internal[0].toUpperCase().replaceAll("_", " ");
	const rest = internal.slice(1).replaceAll("_", " ");

	return `${first}${rest}`;
}

function unformatTitle(formatted) {
	return formatted.toLowerCase().replaceAll(" ", "_");
}

async function main() {
	const searchItems = await (await fetch("/search")).json();
	setSearchResults(searchItems, "");

	elements.searchBox.addEventListener("input", () => {
		const filter = elements.searchBox.value;
		setSearchResults(searchItems, filter);
	});

	formatMath();
}

function setSearchResults(items, filter) {
	elements.searchResults.innerHTML = items
		.filter(item => item.includes(unformatTitle(filter)))
		.map(title => `<a href="/pages/${title}.html">${formatTitle(title)}</a>`)
		.slice(0, 5)
		.join("");
}

function formatMath() {
	if (!window.MathJax) {
		window.MathJax = {
			tex: {
				inlineMath: { '[+]': [['$', '$']] }
			}
		};
	}
	var script = document.createElement('script');
	script.src = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js';
	document.head.appendChild(script);
}

main();

