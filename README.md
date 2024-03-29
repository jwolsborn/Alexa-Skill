# Alexa-Skill
For this I have created an AWS Lambda for my CS410P Rust project at Portland State.

This Lambda is pretty straight forward.  To build this you need to run the following commands

cargo build --release --target x86_64-unknown-linux-musl
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

This zip file contains an executable bootstrop that is then uploaded to the AWS Lambda console.

You then need to Alexa skill that points to the endpoint created by the lambda.  You will then be allowed to name the skill and supply
an invocation name that is used.  I used the invocation name 'blazer skill'.  So statements to Alexa would be something like
"Alexa, ask blazer skill how many points did Damian Lillard score?"  and a response would be invocted.

It's important to note that the user defined intents are programmed into this lambda, so when designing intents the same intent names need
to be used in the Alexa Developer Console.

For testing I simply did manual testing in a video that was included with the turnin of this project as it was difficult to create my own
Alexa request objects as they are sent as JSON's from Amazon.

This project was far more difficult than I had originally expected.  The original goal was to create a webscraper that would seed the lambda with the desired sport stats.  However, when I attempted to combine the two parts I found out that the webscraper crate and the aws
crate both used different versions of the same sub-crate.  This posed a problem.  Due to to the deadline I decided it would be best to just have two parts that worked seperately.  I did enjoy connecting Rust, traditionally known as as system language, to something like AWS which is so pervasive now in our industry.
