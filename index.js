console.log("Quering...")
fetch("/api/search", {
    method: 'POST',
    headers: { 'Content-Type': 'text/plain' },
    body: 'application of ensemble training',
}).then((response) => console.log(response))
