Domo SDK
===

Introducing the newest Domo SDK.
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
   - `export DOMO_API_HOST=https://api.demo.domo.com`
   - `export DOMO_API_CLIENT_ID=your-client-id-here`
   - `export DOMO_API_CLIENT_SECRET=YOURCLIENTSECRETHERE`
   - Alternatively you can set them as flags.
1. You're ready to go

Domo Library
===

You can pull in the library and use the calls directly with your own code

Create the client

	let domo = Domo::new(&app.host, &app.client_id, &app.client_secret);

Use the methods on it

        let r = domo.list_datasets(limit, offset).unwrap();
	

Domo CLI
===

Run the command to get full help.

	domo -h

Currently offers wrappers around the api via sub-commands

	domo dataset list
	domo stream retrieve 4
	domo workflow list
	domo user list-all

If you udpate or create a new object, it will open up an editor so you can modify the object in yaml format.
You can use the env flag DOMO_EDITOR to override which editor is used:

	DOMO_EDITOR=nano dataset create
