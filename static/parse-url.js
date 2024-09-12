export function parseUrl(url) {
	const [_, end] = url.split("public");
	return end;
}
