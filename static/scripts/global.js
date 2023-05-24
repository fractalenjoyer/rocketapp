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
	window.location.href = `/post/${id}`;
};

// // would rather replace this with in-html event listener for clarity
// modal.onclick = (e) => {
// 	e.target.style.display = "none";
// };
