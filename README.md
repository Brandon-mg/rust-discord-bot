### Rust based discord bot

this bot is a project to help me learn Rust while also working on something I've been wanting to make for a long time. uses poise and serenity as discord framework and image (and photon in the future) for image processing.
Right it has only 3 commands:
- slash/prefix commands
	- file_details[attachment]: print message that says if the attached file is an image or not
	- fractal[num]:print a generated fractal using the image crate where num is the background brightness
- prefix only
	- totalsize[vec{attachment}]:print total and avg sizes of all attachments.
	
