# Challenges

You wanna help?  You're so cool ❤ Here's a list of things that need
doing, similar to how an issue tracker would outline them.  We'll try
to keep this list up-to-date, and also always feel free to e-mail the
"community" mailing list for feedback or ask on IRC!


## Filesharing service

That sounds hard, but it's not that hard!  There's no abstractions for
you to introduce because libqaul provides all of them for you! Now, we
might end up removing this abstraction from libqaul again, and moving
it to the service entirely, but that can be done later.

There's "File", "FileMeta", and "FileFilter".  We need to provide
people with an API with which they can announce, send and query for
files.  The service API itself needs to take a file, serialise it with
"conjoiner_engine", the same serialiser user all over libqaul and the
messaging service (just check there how to use it), and send them out
as messages to either the entire network (as a "fileannounce type"),
or just to one person.

When the service get's a "FileRequest" (TODO: add that type to "FileMeta", it needs to load the file from
disk, and then send it.

If you have questions, don't hesitate to ask!


## UDP driver testing 

There's netmod-udp in the qaul.net tree, and while it's mostly (?)
done, it could use some testing.  Best to just ping spacekookie or pbb
on IRC or the mailing list for this, as it's a bit more vague.
