
Requestors Responders
=====================

Requestors-Responders is a convention set built around utilizing thin signals to coordinate multiple physical devices.
It is inspired by behaviour trees, and often lends itself to multitree structures.

BiFi Wire
---------

The convention is built around bidirectional thin-firing wires, one direction being for requests, and the reverse direction being for responses.

Since thin wires can only fire once per instant, BiFiWis can run into the problem of both the requestor and the responder being able to react instantly.
This means that in some situations a requestor or a responder will need to add a device to delay its reacting signal if necessary.
(delay' = x & delay; x' = x | delay;)

Instant Requestor
-----------------

This is a device with a thin input and a requestor terminal, and it sends a request whenever it receives an input OR a response.
If it is asked to send two signals in the same instant, (i.e. the responder responds instantly) then it will delay the second signal, to keep the cycle going.

Instant Responder
-----------------

This is barely a device, it simply takes the input of a response terminal, and feeds it back into the output.

Request Splitting
-----------------

Request splitters are gates that have one response terminal and two or more request terminals.
Some are associative, and so only need to be defined for two outputs.

###Sequence

This device splits a request by first sending the request in one direction, and once it gets a response, sends a request down the second direction; once this responds, it responds to the original request to it.

Implementation of this device is trivial, just wire each input to an output in a ring.

This device is associative, so any tree structure made of sequence splitters is equivalent.

###Full-sync

Synchronized splitters, on the other hand, react to requests by sending the requests to both outputs simultaneously.
Once both respond, the splitter itself will respond.

##Half-sync

The half-syncronized splitter is a variant that responds to the original message as soon as either delegate responds.

Both the full sync and half sync are associative, and can be implemented in a number of ways, depending on how well-behaved the devices to which they connect can be assumed to be.
For example, simply passing back every second response would fulfil the specification of either sync device if its delegates are garanteed not to send double responses.
Handling double requests from the input side is very tricky.

##Allocator

The allocator is a way of splitting a request-response line without splitting any of the individual requests.
If the allocator isn't waiting for its first output to respond, it will send requests to this output
Requests forwarded in this way will immediately receive responses.
Requests received by an allocator while its first output is occupied will be send to a second output.
This request will not be responded to until either of the forwarded requests receive a response.

The allocator thus responds to requests to indicate its ability to receive an additional request.

Buffer
------

The request buffer is a device that can be in three states: 'overloaded', 'full', and 'empty'.
- When it is empty, it passes requests forward, simultaneously responding to the original requestor; the buffer is then full
- When it is full, it does not respond to requests:
 - If it gets an additional request it will become overloaded
 - If it receives a response from the request it originally forwarded, it will become empty
- When it is overloaded, it acts in a reverse manner to the empty state:
 - responses from the original request will be passed backwards
 - the buffer will also immediately send another request forward

The request buffer would in fact be implemented by passing both outputs of an allocator into a \[merge thingy.. buffer merge?\]
