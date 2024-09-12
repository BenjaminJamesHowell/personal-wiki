export function formatTitle(title) {
	const first = title[0].toUpperCase().replaceAll("_", " ");
	const rest = title.slice(1).replaceAll("_", " ");

	return `${first}${rest}`;
}

