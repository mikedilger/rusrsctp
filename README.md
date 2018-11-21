# rusrsctp

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE-APACHE)

usrsctp (used by rusrsctp) is:
[![BSD 3-clause licensed](https://img.shields.io/badge/license-BSD3-blue.svg)](./usrsctp/LICENCE.md)

This library provides rust bindings to usrsctp, a userspace SCTP library which operates either
at the IP layer or over UDP.

## Motivation

For some networking tasks, neither TCP nor UDP provide a good solution. These situations
include voice over IP (VoIP), position updates in games or virtual worlds, and even
multiple parallel resource requests of HTTP over a single connection (this latter case
is being solved via QUIC which implements features similar to SCTP over UDP).

In all of the above situations, data is message oriented and subsequent messages are
not dependent on previous messages.  In VoIP and game update situations subsequent messages
actually override previous ones.

TCP provides a streaming abstraction. This is perfect for streams of data. But it is not
helpful when retransmission and/or reordering delays at some point in the stream
unnecessarily hold up messages further down the stream.  This is called "head of line
blocking" and it cannot be avoided if you use TCP.  Additionally, one would need to implement
delimeters between messages in a stream, and code to break the stream apart into messages,
something not necessary if the transport layer was already message based.

UDP solves all the problems in the previous paragraph.  However, UDP is unreliable and
does not implement flow control, congestion control, explicit congestion notification,
or path MTU discovery, all of which are very useful if not necessary.

SCTP adds these features UDP is missing: flow control, congestion control, explicit
congestion notification, and path MTP discovery. Under SCTP: reliability is optional,
in-order delivery is optional, both streams and messages can be used. SCTP also adds some
more novel features such as multiplexed connections and multihoming.

Due to deployment considerations, SCTP over IP (alongside TCP and UDP) is rarely used.
It does not traverse NAT, and some routers and firewalls don't pass the traffic. Many
system administrators are simply unaware of the protocol, and setup firewalls to pass
TCP and UDP only.  Thus RFC 6951 described how to encapsulate SCTP within UDP.  The
`usrsctp` library implements RFC 6951, as well as RFC 6458 (socket API extensions) and
of course RFC 4960 (SCTP). `rusrsctp` provides rust bindings to that library.

## Warnings

This is early days, not much is implemented yet.

The low level bindings (rusrsctp-sys) were built for linux. Support of other operating
systems (especially Windows) will come later.

This git repository contains a submodule for `usrsctp`, be sure to clone with submodules.
