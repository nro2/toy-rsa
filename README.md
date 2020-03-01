Nick Robinson
CS 410P Rust Programming Winter 2020
1/28/2020
HW 2

This assignment was pretty straight forward.  I started by watching Bart's video on creating libraries in order to figure out exactly how to
tie in another library.  I then created the library, and started translating the pseudocode.  I did pretty much a straight across the board
translation, and when I tried to do a `cargo clippy` on it, I was met with a bunch of proposed `into()`'s and `try_into()`.  I was thinking
just how much I loved Rust when I saw this.  Unfortunately, I ran into some errors.

I met with Jordan on Slack to try to figure some stuff out.  I think that in the long run, throwing the `try_into()` everywhere it told me to
was in fact bandaging a larger problem, and every error we fixed led to another one.  Eventually with Jordan's help, I was able to work through
and get everything functioning.  

The tests that I wrote involved testing that my `genkey()` function was producing numbers that met the desired constraints.  I then set up
a comprehensive test with a fixed message, using the info that Bart provided in the assignment.  I then wrote a test that generated random
messages to test it with whatever number it could throw at it.  All tests passed.

I ran clippy and format to check that there were no errors.

Overall I enjoyed this assignment. I am starting to really like how Rust is so verbose in its error messages.  It makes it easier to learn
than other languages I have tried in the past.  I have only written two programs in Rust now, but I can definitely see the appeal.