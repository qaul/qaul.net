# qaul.net tests

qaul.net is quite a large application, and it's testing surface is
equally large.  While the individual crates have their own tests to
verify that things generally work, integration of different components
can cause other issues as well.


This test suite creates some of these integration tests, mainly, the
rpc interface, the http interface, and webgui tests.  Each test module
is documented separately because setup can vary wildly.


TODO: add a "run-all.sh" or something here...
