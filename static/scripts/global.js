fetch("/api/whoami")
	.then((response) => (response.status === 200 ? response.text() : false))
	.then((data) => {
		if (data) {
			document.querySelector(".profile a").href = `/profile`;
			hello.innerText = data;
		}
	});

const modal = document.querySelector(".modal");
const modalContent = document.querySelector(".modal-content");

const showPost = (id) => {
	// check if user is logged in by fetching /api/whoami
	// if not, redirect to login page
	fetch("/api/whoami").then((res) => res.status)
    .then((status) => {
        if (status === 200) {
            modalContent.src = `/post/${id}`;
            modal.style.display = "flex";
        } else {
            window.location.href = "/login";
        }
    })
};

// would rather replace this with in-html event listener for clarity
modal.onclick = (e) => {
	e.target.style.display = "none";
};
