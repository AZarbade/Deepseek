console.log("Quering...")
fetch("/api/search", {
    method: 'POST',
    headers: { 'Content-Type': 'text/plain' },
    // body: 'dynamic response analysis of tube array in partially filled', // pg.27
    body: 'back analysis of large geotechnical models', // pg.369
    // body: 'evidence of growth of supraspinal axons', // pg.733
}).then((response) => console.log(response))
