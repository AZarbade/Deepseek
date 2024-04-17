async function search(prompt) {
    const resultsContainer = document.getElementById("search-results");

    if (resultsContainer) {
        resultsContainer.innerHTML = ""; // Clear previous results

        const response = await fetch("/api/search", {
            method: 'POST',
            headers: { 'Content-Type': 'text/plain' },
            body: prompt,
        });

        const json = await response.json();
        for ([path, rank] of json) {
            let item = document.createElement("div");
            item.textContent = `${path}`;
            resultsContainer.appendChild(item);
        }
    } else {
        console.error("Search results container not found");
    }
}

const searchBar = document.getElementById("search-bar");
if (searchBar) {
    searchBar.addEventListener("keyup", (event) => {
        if (event.key === "Enter") {
            search(searchBar.value);
        }
    });
} else {
    console.error("Search bar input not found");
}
