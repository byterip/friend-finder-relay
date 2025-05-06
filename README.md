I created this for myself and a few friends to use privately without using the public endpoint everyone else is on, but you're welcome 
to use it anyway you see fit just don't expect great support if you don't know what you're doing. Service is being hosted on [Shuttle.dev](https://www.shuttle.dev/) but can be
easily deployed anywhere by just swapping out the Shuttle boilerplate.

Goals of this service:
* All data is stored in memory and not in a database somewhere.
* Can be easily deployed on just about any platform.
* Fast enough to handle hundreds if not thousands of requests from many players using the same endpoint every few seconds.

Not implemented yet:
* Authentication (Friends API Key setting in the actual RuneLite plugin)
* Reports & Server Location endpoints (no current plans to add or even look into these)

RuneLite plugin this relay targets:
https://runelite.net/plugin-hub/show/friend-finder
