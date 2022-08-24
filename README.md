# CAFHS notetaking application

This repository contains the backend code for a small web-based notetaking application made for the South Australian branch of the Child and Family Health Services (CaFHS), created as the final (external) assessment for SACE stage two Digital Technologies in 2022.

The current goals for this application include:
- [ ] Interaction with a MongoDB database to store notes and client/user credentials
	- Serialize / Deserialize data into Rust structs to simplify manipulation of data
	- Add / Update / Remove documents
- [ ] Authentication of Users to prevent unauthorized access to notes
	- Password encryption
	- Webpage authentication based on user priviledge
- [ ] Serving of a web-ui and api endpoints using [Rocket.rs](https://rocket.rs/)
