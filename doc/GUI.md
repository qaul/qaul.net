GUI - Graphic User Interface
============================

The qaul.net GUI is a HTML GUI, that runs in a `WebView`. The GUI uses the 
[jQuery Mobile](http://jquerymobile.com/) framework.

When qaul.net application is running one can access the GUI via web a
browser on port 8081: 
[http://localhost:8081/qaul.html](http://localhost:8081/qaul.html)


Web client & Captive Portal
---------------------------

When connecting to qaul.net via wifi without running the qaul.net software 
the device gets configured with an IP address and a captive portal DNS 
address.

* captive portal start page with software download function
  * source [GUI/www/index.html](GUI/www/index.html)
  * access page with web browser when running qaul.net 
    [http://localhost:8081/](http://localhost:8081/)
* web client 
  * source [GUI/www/qaul_web.html](GUI/www/qaul_web.html)
  * access web client with web browser when running qaul.net
    [http://localhost:8081/qaul_web.html](http://localhost:8081/qaul_web.html)


GUI Development
---------------

All GUI files are located in the directory 
[GUI/www](GUI/www). 
The whole GUI is all in a single web page: 
[GUI/www/qaul.html](GUI/www/qaul.html).

To test and develop the GUI it is good practice to copy all files from the `www/` directory
to your local web directory. To test the user interface on your local 
web server copy the test files from 
[GUI/www_GUI-test_static](GUI/www_GUI-test_static) 
to your the web directory.

    cp GUI/www /PathToYourLocalWebDirectory
    cp GUI/www_GUI-test_static /PathToYourLocalWebDirectory

Now go to the page `qaul.html` in your web browser. 
