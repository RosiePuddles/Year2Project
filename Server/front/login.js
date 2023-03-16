(() => {
	'use strict'
	const form = document.getElementById('form')
	form.addEventListener('submit', event => {
		if (!form.checkValidity()) {
			event.preventDefault();
			event.stopPropagation();
		}
		form.classList.add('was-validated');
	}, false)
})()

const sww = "Something went wrong! Please try again later"

window.addEventListener("load", () => {
	try {
		keccak_512("")
	} catch {
		alert(sww);
		console.error("")
		setTimeout(() => {
			window.location.replace(`${location.protocol}//${location.host}`)
		}, 2000)
	}

	function sendData() {
		const req = new XMLHttpRequest();
		const FD = new FormData(form);


		req.addEventListener("load", (event) => {
			switch (event.target.status) {
				case 200:
					const resp = JSON.parse(event.target.responseText);
					let t = new Date();
					t.setTime(Date.parse(resp.cookie.time));
					document.cookie = `login=${resp.cookie.value};expires=${t.toUTCString()}`;
					window.location.replace(`${location.protocol}//${location.host}/${resp.redirect}`);
					break
				case 400:
					alert(sww)
					console.error(`Form deserialization error ${event.target.responseText}`)
					break
				case 500:
					alert(sww)
					console.error('Server error')
					break
				case 404:
					alert(sww)
					console.error('Submit POST request not found')
					break
				case 403:
					// bad email
					document.getElementById("form_email").valid = false
					document.getElementById("email-feedback").innerText = "Please enter an email address"
					break
			}
			console.log(event.target);
		});

		req.addEventListener("error", (_) => {
			alert(sww);
		});

		req.open("POST", location.protocol + '//' + location.host + location.pathname);

		req.setRequestHeader("content-type", "application/json")
		req.send(JSON.stringify({
			"email": FD.get("email"),
			"password": keccak_512(FD.get("password"))
		}));
	}

	const form = document.getElementById("form");

	form.addEventListener("submit", (event) => {
		event.preventDefault();
		sendData();
	});
});
