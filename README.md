# gamer - Game-Agnostic Matchmaking Engine for Rust

# Matchers

Matcher is a tool to find out what game should be played next.

There are two types of matchers: simple and continuous.

## Simple

Simple matchers have fixed input set, and they create a fixed number of games.
For example `RoundRobin` matcher always generates `(0..n).sum()` games from `n`
participants.

## Continuous

Continuous matchers create an infinite stream games. They may also have dynamic
input set, that allows adding (and removing) participants. Continuous matcher
can also accept feedback, e.g. score or result of a match, to select future
pairs.
