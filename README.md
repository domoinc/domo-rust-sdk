Domo SDK
===

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/domo">
    <img src="https://img.shields.io/crates/v/domo.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/async-std">
    <img src="https://img.shields.io/crates/d/domo.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/domo">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

Introducing the best Domo SDK.
We are happy to help integrate your rust code and processes into Domo.
Welcome to the Rust Domo SDK.
This project aims to help developers build great Rust solutions for Domo.

This project contains a binary cli application in additon to a library.

Getting Started
===

Domo offers a public API to Domo users that want to develop solutions against their instance.

1. Go to the [developer portal](https://developer.domo.com)
1. Sign-In
1. Go to the [my client](https://developer.domo.com/manage-clients) page
1. Create a client with the scopes desired from the following:
   - account
   - audit
   - buzz
   - data
   - dashboard
   - user
   - workflow
1. [CLI] Set your environment up to use your client_id and client_secret
   - You can set them in your .bash_profile to keep them persistant
   - `export DOMO_API_HOST=https://api.domo.com`
   - `export DOMO_API_CLIENT_ID=your-client-id-here`
   - `export DOMO_API_CLIENT_SECRET=your-client-secret-here`
   - Alternatively you can set them as flags.
1. You're ready to go

Domo Library
===

You can pull in the library and use the calls directly with your own code

Create the client

	let domo = Domo::new(&app.host, &app.client_id, &app.client_secret);

Note: Please do not check your credentials into your code.
We recommend using environment variables or other config that is not in your repo.

Use methods on the provided client

  let r = domo.list_datasets(limit, offset).await.unwrap();
	

Domo CLI
===

Run the command to get full help.

	domo -h

Currently offers wrappers around the api via sub-commands

	domo dataset list
	domo stream retrieve 4
	domo workflow list
	domo user list-all

If you update or create a new object, it will open up an editor so you can modify the object in yaml format.
You can use the env flag DOMO_EDITOR to override which editor is used:

	DOMO_EDITOR=nano dataset create

Updates
===

2021-03-17 Version 0.3.2
---

* Updated libraries to latest
* Cleaned up clippy lint suggestions

2021-01-31 Version 0.3.1
---

* Updated surf to 2
* Bugfix on dataset API
* Updated dependencies to latest
* Thanks to @ryanmurf and @TrashPandacoot for contributions

2020-09-15 Version 0.2.0
---

* Updated to use the [surf](https://crates.io/crates/surf) library to offer the library as async
* Due to this there is some functionality with uploading attachments in workflows that are not working
* The binary now runs on async as well
* Provided an error object for better error output including toes to send to support.