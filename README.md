# Overview

This software uses an API to get a youtube channel and look at the last 5 videos uploaded and outputs the name of the video and when it was output. I wrote this code because I had never used an API before and it was a good way to learn how to use them, I still have many things to do with this code to make it better.

# Development Environment

I used serde, reqwest and dotenv in this program to read json outputs.

# Useful Websites

* [rust-lang.org](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok)
* [Stackoverflow](https://stackoverflow.com/questions/56279350/how-to-get-at-one-particular-item-in-json-file-using-serde-json-without-deriving)

# Future Work

* Make the output cleaner so that it is easier to understand.
* Add a file so that the requests that I have taken so far will save so I do not send 20 requests to get the same stuff each time.
* Add a file to save channel names and channel codes for channels that I frequently look up.