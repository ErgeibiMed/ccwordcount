# WordCount Command Line Tool

This is simply an exercise from Build Your Own wc Tool from
[coding challenges website](https://codingchallenges.fyi/challenges/challenge-wc/)

# Description

This is my implementation in rust of the challenge build your own version of the Unix command line tool wc!

The Unix command line tools are a great metaphor for good software engineering and they follow the Unix Philosophies of:

   - Writing simple parts connected by clean interfaces - each tool does just one thing and provides a simple CLI
   that handles text input from either files or file streams.
   - Design programs to be connected to other programs - each tool can be easily connected to other tools to create
   incredibly powerful compositions.

Following these philosophies has made the simple unix command line tools some of the most widely used software
engineering tools - allowing us to create very complex text data processing pipelines from simple command line tools.
You can read more about the Unix Philosophy in the excellent book
[The Art of Unix Programming](http://www.catb.org/~esr/writings/taoup/html/).

# Usage
 - Cargo run -- -flag filename
 - ccwc -flag filename
 - flag is one of the following:
  -- c : for number of bytes in the file
  -- l : for number of lines in the file
  -- w : for number of words in the file
  -- m : for number of characters in the file

# Example
 cargo run -- -c test.txt // should output : 342190 test.txt


# Dependencies
 [clap](https://github.com/sharkdp/clap-rs)

# License
Please read the [LICENSE-MIT](https://github.com/ErgeibiMed/ccwordcount/blob/main/LICENSE.txt) file in this repository for more information.
