const log_out = () => {
	document.cookie = "login=;expires=Thu, 01 Jan 1970 00:00:00 UTC;"
	window.location.replace(`${location.protocol}//${location.host}/login`);
}