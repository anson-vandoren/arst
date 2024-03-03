# `arst`, at least until I think of a real name

I was looking at [devutils](devutils.com) and thinking it would be nice if

1. I could use it from the command line
2. It worked on Linux
3. It was open source

So, here we go. To be clear, I don't actually care about a lot of the features
it includes, and I have several others that aren't included in it, but the idea
of a simple tool for conversions, generations, and formatting is appealing.

Yes, there are tools for all these things, but having a consistent syntax and
one tool seems convenient.

# Things I probably want

- [ ] JSON pretty print
- [ ] JSON -> YAML -> JSON
- [ ] Line sort & dedupe
- [ ] Number base conversions
- [ ] LEB128 conversions
- [ ] UUID generation
- [ ] Hex -> ASCII -> Hex
- [ ] Bytes in almost any format to:
  - [ ] Base64 string
  - [ ] JS Array of numbers
  - [ ] ASCII
  - [ ] Hex string
- [ ] Unix time to:
    - [ ] Relative time
    - [ ] Human readable time (wtf does this mean?)
    - [ ] ISO 8601
    - [ ] And from these back to Unix time, and like such as
- [ ] MD5, SHA1, SHA256, SHA512
- [ ] JWT decode
- [ ] URL encode/decode
- [ ] X.509 certificate parsing
- [ ] Escape/unescape JSON
- [ ] Cron expression parsing

# Design goals & constraints

- [ ] Single, small binary
- [ ] Cross platform
- [ ] Rust
- [ ] Intuitive (to me) syntax
- [ ] Man page
- [ ] Shell completions
- [ ] Tolerant of input
- [ ] Fast
