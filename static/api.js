export async function getPage(url) {
	const response = await fetch(`/api${url}`);
	const json = await response.json();

	if (json.status === "Err") {
		throw json;
	}

	return json.body;
}

