# RBlitz

[![Build Status](https://travis-ci.com/Veykril/rblitz.svg?branch=staging)](https://travis-ci.com/Veykril/rblitz)

A soon to be League of Legends Season 2 Server written in Rust.
League makes heavy use of Inheritance, and since Rust does not have Inheritance, an ECS system called [Specs](https://github.com/slide-rs/specs) was used to solve this problem. All the behaviours will therefore be implemented as [Systems](https://docs.rs/specs/latest/specs/trait.System.html) with the exception of Packet Handling and Sending, they will be implemented manually due to execution order and the inability to parallelize the execution of them. Below is a(n) (incomplete) list of Systems that are already/still have to be implemented:

### Systems:
- [x] packet handler(*)
- [x] packet sender
- [ ] vision
- [ ] pathing

*the packet handler system actually consists of many packet handlers that are each their own systems 

# License

This software is licensed under the GNU General Public License version 3. You can use, copy, modify
the software as long as all the derivative work is published under the same license. A copy of the
license can be found in the [LICENSE][license] file of the repository where a detailed
explanation of the copying rights can be found.

[license]: /LICENSE
