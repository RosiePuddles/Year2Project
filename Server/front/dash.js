(() => {
	'use strict'
	const form = document.getElementById("form")
	form.addEventListener('submit', event => {
		if (!form.checkValidity()) {
			event.preventDefault();
			event.stopPropagation();
		}
		form.classList.add('was-validated');
	}, false)
})()

const start_date = document.getElementById("start_date")
const end_date = document.getElementById("end_date")
const form = document.getElementById("form")

const sww = "Something went wrong! Please try again later"

const log_out = () => {
	document.cookie = "login=;expires=Thu, 01 Jan 1970 00:00:00 UTC;"
	window.location.replace(`${location.protocol}//${location.host}/login`);
}

const date_form_string = (d) => {
	return `${String(d.getFullYear()).padStart(4, "0")}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}T${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`
}

const check_start = () => {
	if (end_date.value === "" || Date.parse(end_date.value) < Date.parse(start_date.value)) {
		let d = new Date()
		d.setTime(Date.parse(start_date.value) + 86400000)
		end_date.value = date_form_string(d)
	}
}
const check_end = () => {
	if (start_date.value === "" || Date.parse(end_date.value) < Date.parse(start_date.value)) {
		let d = new Date()
		d.setTime(Date.parse(end_date.value) - 86400000)
		start_date.value = date_form_string(d)
	}
}

start_date.addEventListener("input", check_start);
end_date.addEventListener("input", check_end);

const sendData = () => {
	const req = new XMLHttpRequest();
	const FD = new FormData(form);


	req.addEventListener("load", (event) => {
		switch (event.target.status) {
			case 200:
				const resp = JSON.parse(event.target.responseText);
				const download = document.createElement("a");
				download.href = resp.redirect;
				download.download = resp.name;
				document.body.appendChild(download);
				download.click();
				document.body.removeChild(download)
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
		}
	});

	req.addEventListener("error", (_) => {
		alert(sww);
	});

	req.open("POST", location.protocol + '//' + location.host + location.pathname);

	req.setRequestHeader("content-type", "application/json")
	let d = new Date()
	d.setTime(Date.parse(FD.get("start")))
	const start = d.toISOString()
	d.setTime(Date.parse(FD.get("end")))
	const end = d.toISOString()
	req.send(JSON.stringify({
		"start": start,
		"end": end,
		"format": FD.get("format")
	}));
}

form.addEventListener("submit", (event) => {
	event.preventDefault();
	sendData();
});
