async function search(prompt) {
  const results = document.getElementById("search-results");
  results.innerHTML = ""; // Clear previous results

  const response = await fetch("/api/search", {
    method: 'POST',
    headers: { 'Content-Type': 'text/plain' },
    body: prompt,
  });

  const json = await response.json();

  for ([path, rank] of json) {
    let item = document.createElement("div");
    item.textContent = `${path}`;
    results.appendChild(item);
  }
}

const searchBar = document.getElementById("search-bar");
searchBar.addEventListener("keyup", (event) => {
  if (event.key === "Enter") {
    search(searchBar.value);
  }
});
