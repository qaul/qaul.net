/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


// ======================================================
// Captive Portal Whitelisting
//
// Mobile devices need whitelisting to use networking
// outside the captive portal.
// ------------------------------------------------------

// contiously whitelist this IP
var whitelist_captive_portal = function()
{
	var path = "/whitelist.json";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			// renew the whitelisting in 15 seconds
			setTimeout(function(){whitelist_captive_portal();},15000);
		}
	}).error(function(){
		// renew the whitelisting in 15 seconds
		setTimeout(function(){whitelist_captive_portal();},15000);
	});
}

// ======================================================
whitelist_captive_portal();
