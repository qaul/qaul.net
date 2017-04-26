Community Template
==================

This folder contains qaul.net's community templates. Community templates
contain the network configuration definition of a Wifi mesh community. 
Every Wifi community that is using OLSRD as its routing protocol is 
compatible with the qaul.net app.
Users of qaul.net can configure the app to communicate with the local 
Wifi mesh network via the GUI: 
"Preferences Tab > Configure Custom Network"

Every community configuration is defined in a single JSON file in the 
folder [GUI/www/community-templates](.)


JSON Structure Explanation
--------------------------

Use an existing community profile as the bases for your community template e.g.
[GUI/www/community-templates/qaul.json](qaul.json)


* profile:
  * name: The name of your wifi community
  * homepage: A link to your community home page
  * info: More information for the user on how to interact with your 
    wifi community. e.g. if users need to register an IP first, inform 
    the user about it and describe how to do that.
* ip: the IPv4 address of the users node. 
  * generate: A random IP addresses can be created for the user when the 
    value is "true". No address is generated if the value is "false".
  * pattern: The pattern of the IP address to be automatically generated. 
    The value has to IPv4 with a change mask after the slash e.g. 
    "10.0.0.0/24"
* channel: The wifi channel number of your wifi community.
* ssid: the SSID of your wifi mesh community.
* bssid: Contains the BSSID of your wifi ad-hoc cell, if you use any. 
  This is only supported by very few Linux drivers.


You can define for every value whether it is displayed to the user and 
whether it can be edited. The behaviour is defined by the "edit" key. 
The possible values are:

* "true"  (the configuration option is diplayed, and the user can configure it)
* "false" (the configuration option is diplayed but the user can't change it)
* "hidden" (the configuration option is hidden on the configuration page, and the user can't change it)


How to add a Community Template
-------------------------------

1. Create a new json file and name it after your community e.g. 
   [GUI/www/community-templates/your-community.json](custom.json)
2. Fill in the values
3. Add your file as an option tag to the file 
   [GUI/www/qaul.html](../qaul.html) in 
   `<select name="profile" id="c_network_profile">` e.g. 
   `<option value="yourCommunity">Your Wifi Community Name</option>`
4. Create a pull request on Github to get your template included in 
   qaul.net

