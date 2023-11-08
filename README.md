# help desk bot

this is a telegram bot designed to be an FAQ system.
it started on [Imaguru](https://imaguru.lt/) hackathon in Vilnius back in 2021. The version implemented during that hackathon was written in python and is available [here](https://github.com/verrchu/vilnius_help_desk).
the idea was to gather all the usefule information for recent immigrants to Lithuania in that bot. the project won the hackathon and we decided to continue developing it.

later it ws rewritten in rust completely. It used redis as its primary storage (for no good reason except that I wanted to try this approach) and hosted on AWS.
soon "leave feedback" feature was added and the feedback was  collected using another tiny bot  available [here](https://github.com/verrchu/help_desk_feedback_bot)

the main bot and the feedback bot were linked via [redis streams](https://redis.io/docs/data-types/streams/)

unfortunately after a while the project got abandoned and it is unfortunate. looking at it now I can see that we even got some positive feedback from the users via that "leave feedback" feature
