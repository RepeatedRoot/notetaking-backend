//database url
const DATABASE_URL = "http://192.168.0.20:8000";

/* Declaring references to main bootstrap components */
const loginModal = new bootstrap.Modal("#loginModal");
const noteModal = new bootstrap.Modal("#noteModal");
const clientModal = new bootstrap.Modal("#clientModal");
const messageToast = new bootstrap.Toast(
	document.getElementById("messageToast")
);

//Append a client entry to the list
$.fn.appendClient = function (clientdata) {
	"use strict";
	let clientList = $(this); //The ID of the client list div

	//Add the HTML string of the entry to the div, inserting information about the client
	clientList.append(
		`<a id=${clientdata._id.$oid} notes=${clientdata.notes.$oid} class="client-entry list-group-item list-group-item-action py-3 lh-sm" aria-current="true" style="cursor: pointer"	>
			<div class="d-flex w-100 align-items-center justify-content-between">
				<strong class="mb-1">${clientdata.firstname} ${clientdata.surname}</strong>
				<small>${clientdata.sex}</small>
			</div>
			<div class="col-10 mb-1 small">${clientdata.address}</div>
		 </a>`
	);
};

/* Get list of clients from the database */
$.fn.getClients = function () {
	"use strict";

	var response; //to store the JSON object

	/* AJAX GET request to the client API */
	$.ajax({
		url: `${DATABASE_URL}/clients/`,
		method: "GET",
		async: false,
		xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
		success: function (data) {
			response = data;
		},
		error: function (error) {
			console.log(error);
		},
	});

	/* Append each entry in the list to the client container */
	response.forEach((client) => {
		this.appendClient(client);
	});

	/* Add an event listener to each of the client entries which will display the client's notes */
	$(".client-entry").click(function () {
		$(this).viewNotes();
	});
};

/* Render notes */
$.fn.viewNotes = function () {
	"use strict";

	let client = $(this); //The ID of the client information div

	let NoteId = client.attr("notes"); //The note ID attribute of the client entry

	$(".notes-container").attr("current-client", client.attr("id"));
	$(".notes-container").attr("current-notes", client.attr("notes"));

	$(".notes-container .row").remove(); //remove any old entries if present

	var response; //to store returned information

	/* Request the notes entry for the client from the database */
	$.ajax({
		url: `${DATABASE_URL}/notes/${NoteId}`,
		method: "GET",
		async: false,
		xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
		success: function (data) {
			response = data;
			console.log(data);
		},
		error: function (error) {
			console.log(error);
		},
	});

	/* Append each note to the website */
	response.notes.forEach((note) => {
		$(".notes-container").renderNote(note);
	});
};

/* Rendering notes to the website */
$.fn.renderNote = function (note) {
	"use strict";

	const container = $(this); //the parent element to insert notes into

	/* Parse the datetime object to a string */
	let date = new Date(note.datetime);

	/* Append the HTML container */
	container.append(
		`
		<div class="row p-3">
			<div class="col align-self-center">
				<div class="card">
					<div class="card-header">
						${date.toDateString()}
					</div>
					<div class="card-body">
						<blockquote class="blockquote mb-0">
							<p>${note.note}</p>
							<footer class="blockquote-footer">${
								note.clinician.$oid
							}<cite title="Source Title"></cite></footer>
						</blockquote>
					</div>
				</div>
			</div>
		</div>
		`
	);
};

/* Creating a new note to be appended in the database */
$.fn.createNote = function () {
	"use strict";

	/* Attribute to store the ID of currently selected client's notes */
	let clientNotes = $(".notes-container").attr("current-notes");

	/* Values of inputs elements in the notes modal */
	let date = $("#noteDate").val();
	let note = $("#noteText").val();

	/* PUT request to insert a notes */
	$.ajax({
		url: `${DATABASE_URL}/notes/${clientNotes}`,
		method: "PUT",
		async: false,
		xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
		data: JSON.stringify({
			//Encode data into a json object
			datetime: date,
			note: note,
		}),
		success: function () {
			$().showMessage("Successfully created note", "success"); //Display a success message

			noteModal.hide(); //Hide the notes modal (it is no needed)

			$().viewNotes(); //Re-render the notes to update any changes
		},
		error: function (error) {
			console.log(error);
		},
	});
};

/* Updating or creating client accounts */
$.fn.manageAcc = function (exists) {
	"use strict";

	let firstname = $("#clientFirstName");
	let surname = $("#clientSurname");
	let middlenames = $("#clientMiddleNames");
	let sex = $("#clientSex");
	let address = $("#clientAddress");
	let postal_address = $("#clientPostalAddress");
	let phone = $("#clientPhone");

	if (exists) {
		//The client exists (Update their information)
		/* Yet to be implemented */
		console.log("Updating user information is yet to be implemented");
	} else {
		//The client does not exist (create a new account)
		$.ajax({
			url: `${DATABASE_URL}/client`,
			method: "POST",
			async: false,
			xhrFields: { withCredentials: true }, //This endpoint is restricted, send cookies to authenticate
			data: JSON.stringify({
				firstname: firstname,
				surname: surname,
				middlenames: middlenames,
				sex: sex,
				address: address,
				postal_address: postal_address,
				phone: phone,
			}),
			success: function (result) {
				console.log(result);
			},
			error: function (error) {
				console.log(error);
			},
		});
	}
};

/* Show a message using a bootstrap toast */
$.fn.showMessage = function (message, state) {
	"use strict";

	let colour = "white";

	switch (state) {
		case "danger":
			colour = "red";
		case "warning":
			colour = "orange";
		case "success":
			colour = "green";
	}

	$("#messageToast rect").attr("fill", colour); //Update the colour of the message toast
	$(".toast-body").text(message); //Update the messageToast's message

	messageToast.show();
};

/* Login to the website */
$.fn.login = function () {
	"use strict";

	/* Input values from the login modal */
	/* let username = $('#loginEmail').val();
	let password = $("#loginPassword").val(); */

	let username = "erikaodinson@cahfs.sa.gov.au";
	let password = "password";

	/* POST request to authenticate with the backend (and gain a private cookie if authorised) */
	$.ajax({
		url: `${DATABASE_URL}/login`,
		method: "POST",
		async: false,
		data: JSON.stringify({
			//JSON encode input data
			username: username,
			password: password,
		}),
		success: function (data) {
			loginModal.hide(); //Hide the login modal (login was successful)
			$().showMessage("Successfully logged in.", "success");

			$(".client-list").getClients(); // Display the list of clients
		},
		error: function (error) {
			$().showMessage("There was an error logging in.", "danger");
			console.log(error);
		},
	});
};

/* Logout from the website */
$.fn.logout = function () {
	"use strict";

	/* POST request to the backend to remove the authenticated private cookie */
	$.ajax({
		url: `${DATABASE_URL}/logout`,
		method: "POST",
		async: false,
		xhrFields: { withCredentials: true }, //ensure that cookie is sent
		success: function (data) {
			console.log("Successfully logged out " + data);
		},
		error: function (error) {
			console.log("Error logging out");
		},
	});
};
