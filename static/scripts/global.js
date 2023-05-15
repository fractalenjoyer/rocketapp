fetch("/api/whoami")
.then(response => response.status === 200 ? response.text() : false)
.then(data => {
    if (data) {
        document.querySelector(".profile a").href = `/profile`
        hello.innerText = data
    }
})