/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

// ======================================================
// global definitions
// ------------------------------------------------------
var chat_form;
var msg;
var msg_last_id = 0;
var user_name = "";
var starting;
var chat;
var users;
var name_form;
var count = 0;
var tag_last_id = 0;
var tag_name = "";
var user_last_id = 0;
var qaul_config;
var node_count = 0;
var user_count = 0;

var qaulfiles = [];
var qaulusers = [];
var qaul_initialized = false;
var chat_initialized = false;
var config_network_profile = {};
var is_chrome = false;
var call_page_origin = "page_chat";
var user_page_origin = "page_users";

var QAUL_FILESTATUS_COPYING     = -5;
var QAUL_FILESTATUS_DELETED     = -2;
var QAUL_FILESTATUS_ERROR       = -1;
var QAUL_FILESTATUS_NEW         =  0;
var QAUL_FILESTATUS_DISCOVERING =  1;
var QAUL_FILESTATUS_DISCOVERED  =  2;
var QAUL_FILESTATUS_DOWNLOADING =  3;
var QAUL_FILESTATUS_DOWNLOADED  =  4;
var QAUL_FILESTATUS_MYFILE      =  5;

// ======================================================
// initialize
// ------------------------------------------------------

function init_start()
{
	$.mobile.changePage('#page_loading');
	setTimeout(function(){loadingtimer();}, 1000);
	
	// bugfix check if browser is chrome
	is_chrome = /chrome/.test(navigator.userAgent.toLowerCase());
	
	// declarations
	chat_form=$('#chat_form');
	msg=$('#chat_msg');
	starting=$('#starting');
	chat=$('#chat');
	users=$('#users');
	name_form=$('#name_form');
	
	// events
	$(document).bind("pagechange", onPageChange);
	$(document).bind("pagebeforechange", onPageBeforeChange);
	
	$("#interface_select_auto").change(config_interface_toggle);
	$("#c_internet_share").change(config_internet_toggle);
	$("#c_network_profile").change(config_network_change);
	$("#c_files_autodownload_select").change(config_files_auto_toggle);
	
	// add custom validation method
	jQuery.validator.addMethod("nospaces", function(value, element) { 
		return this.optional(element) || /^[^\s]+$/.test(value); 
	}, "Spaces are not allowed in the user name");

	jQuery.validator.addMethod("userlen", function(value, element) { 
		return this.optional(element) || utf8ByteCount(value)<=20; 
	}, "User name is too long");
	
	jQuery.validator.addMethod("chatlen", function(value, element) { 
		return this.optional(element) || utf8ByteCount(value)<=140; 
	}, "Message is too long");

	jQuery.validator.addMethod("filedesclen", function(value, element) { 
		return this.optional(element) || utf8ByteCount(value)<=80; 
	}, "Description is too long");

	// message forms
	chat_form.validate({
		submitHandler: function(form){
			send_msg();
		}
	});
	
	$("#tag_chat_form").validate({
		submitHandler: function(form){
			send_tag_msg();
		}
	});
	
	$("#user_chat_form").validate({
		submitHandler: function(form){
			send_direct_msg();
		}
	});
	
	// set locale
	$("#locale_submit").click(function(){
		send_locale();
		return false;
	});
	
	// set username
	name_form.validate({
		submitHandler: function(form){
			send_name();
		}
	});
	
	// configure interface
	$("#page_config_interface").on("pagebeforeshow",function(event){
		config_interface_load_data();
	});
	$("#form_config_interface").submit(function(event){
		config_interface_send();
		return false;
	});
	// configure internet
	$("#page_config_internet").on("pagebeforeshow",function(event){
		config_internet_load_data();
	});
	$("#form_config_internet").submit(function(event){
		config_internet_send();
		return false;
	});
	// configure network
	$("#form_config_network").validate({
		submitHandler: function(form){
			config_network_send();
		}
	});
	// configure files
	$("#form_config_files").submit(function(event){
		config_files_send();
		return false;
	});
	
	// files
	$("#file_add_form").validate({
		submitHandler: function(form){
			send_file_add();
		}
	});
	
	// ------------------------------------------------------
	// for msie < 9 compatibility (because jqm breaks the onsubmit event)
	if($.browser.msie && $.browser.version < 9)
	{
		// message forms
		$("#chat_form input").keypress(function(e){
			if(e.which == 13)
			{
				if($("#chat_form").valid())
					send_msg();
				e.preventDefault();
				return false;
			}
		});
		$("#chat_submit").click(function(){
			if($("#chat_form").valid())
				send_msg();
			return false;
		});
		
		$("#tag_chat_form input").keypress(function(e){
			if(e.which == 13)
			{
				if($("#tag_chat_form").valid())
					send_tag_msg();
				e.preventDefault();
				return false;
			}
		});
		$("#tag_chat_submit").click(function(){
			if($("#tag_chat_form").valid())
				send_tag_msg();
			return false;
		});
		
		$("#user_chat_form input").keypress(function(e){
			if(e.which == 13)
			{
				if($("#user_chat_form").valid())
					send_direct_msg();
				e.preventDefault();
				return false;
			}
		});
		$("#user_chat_submit").click(function(){
			if($("#user_chat_form").valid())
				send_direct_msg();
			return false;
		});
		
		// set username
		$("#name_form input").keypress(function(e){
			if(e.which == 13)
			{
				if($("#name_form").valid())
					send_name();
				e.preventDefault();
				return false;
			}
		});
		$("#name_submit").click(function(){
			if($("#name_form").valid())
				send_name();
			return false;
		});
		
		// configure interface
		$("#form_config_interface input").keypress(function(e){
			if(e.which == 13)
			{
				config_interface_send();
				e.preventDefault();
				return false;
			}
		});
		$("#c_interface_submit").click(function(){
			config_interface_send();
			return false;
		});
		// configure internet
		$("#form_config_interface input").keypress(function(e){
			if(e.which == 13)
			{
				config_internet_send();
				e.preventDefault();
				return false;
			}
		});
		$("#name_submit").click(function(){
			config_internet_send();
			return false;
		});
		// configure network
		$("#form_config_network input").keypress(function(e){
			if(e.which == 13)
			{
				if($("#form_config_network").valid())
					config_network_send();
				e.preventDefault();
				return false;
			}
		});
		$("#c_network_submit").click(function(){
			if($("#form_config_network").valid())
				config_network_send();
			return false;
		});
		// configure file sharing
		$("#form_config_files input").keypress(function(e){
			if(e.which == 13)
			{
				config_files_send();
				e.preventDefault();
				return false;
			}
		});
		$("#c_files_submit").click(function(){
			config_files_send();
			return false;
		});
	
		// files
		$("#file_add_form input").keypress(function(e){
			if(e.which == 13)
			{
				if($("#file_add_form").valid())
					send_file_add();
				e.preventDefault();
				return false;
			}
		});
		$("#file_add_submit").click(function(){
			if($("#file_add_form").valid())
				send_file_add();
			return false;
		});
	}
	
	qaul_initialized = true;
	init_config();
}

function init_config()
{
	// load configuration
	$.ajax({
		url:   "getconfig.json",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			qaul_configure(data);
		}
	});
}

function qaul_configure(data)
{
	qaul_config = data;
	
	// set up everything
	if(qaul_config.c_quit) 
		$(".c_quit").show();
	if(qaul_config.c_debug) 
		$(".c_debug").show();
	if(qaul_config.c_interface) 
		$(".c_interface").css("display","block");
	if(qaul_config.c_internet) 
		$(".c_internet").css("display","block");
	if(qaul_config.c_network) 
		$(".c_network").css("display","block");
	
	if(qaul_config.locale)
	{
		// load locale
		$.ajax({
			url:   "i18n/" +qaul_config.locale +".json",
			cache: false, // needed for IE
			dataType: "json",
			success: function(data){
				qaul_translate(data);
			}
		}).error(function(){
			alert("i18n download error");
		});
		
		// download language specific css
		if (document.createStyleSheet){
			document.createStyleSheet('i18n/' +qaul_config.locale +'.css');
		}
		else {
			$("head").append($("<link rel=\"stylesheet\" href=\"i18n/" +qaul_config.locale +".css\" type=\"text/css\" media=\"screen\" />"));
		}
	}
}

function qaul_translate(dictionary)
{
	$.i18n.load(dictionary);

	// check for all i18n classes
	$("a.i18n").each(function(){
		$(this).text($.i18n._($(this).text()));
	});
	$("li.i18n").each(function(){
		$(this).text($.i18n._($(this).text()));
	});
	$("label.i18n").each(function(){
		$(this).text($.i18n._($(this).text()));
	});
	$("input.i18n").each(function(){
		$(this).val($.i18n._($(this).val()));
	});
	$("h1.i18n").each(function(){
		$(this).text($.i18n._($(this).text()));
	});
	$("p.i18n").each(function(){
		$(this).text($.i18n._($(this).text()));
	});
	
	// translate validation msgs
	jQuery.extend(jQuery.validator.messages, {
		required: $.i18n._("This field is required"),
		nospaces: $.i18n._("Spaces are not allowed in the user name"),
		userlen: $.i18n._("User name is too long"),
		chatlen: $.i18n._("Message is too long"),
		filedesclen: $.i18n._("Description is too long")
	});
	// translate search
	$("ul#users").data("filter-placeholder",$.i18n._("Filter items"));
}

function init_chat()
{
	chat_initialized = true;
	
	// show back buttons
	$(".c_init").show();
	
	// todo: put all those timers into one timer
	// set timer
	updatetimer();
	
	// set name
	// load configuration
	$.ajax({
		url:   "getname.json",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			set_username(data.name);
		}
	});

	// load files
	file_update();
	
	// check for global events
	eventstimer();
}

function init_favorites()
{
	var path = "fav_get.json";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			favorites_append(data);
		} 
	}).error(function(){
			// todo: show error page
	});	
}

function set_username(name)
{
	user_name = name;
	$("#chat_name").val(user_name);
	$("#page_pref_name").text(user_name);
}

function set_wifiset()
{
	var path = "set_wifiset.json";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			$.mobile.changePage($("#page_loading"));
			setTimeout(function(){loadingtimer();},1000);
		} 
	});		
}

// ======================================================
// change views
// ------------------------------------------------------

function show_user(name, ip, id)
{
	user_last_id = 0;

	// open page
	if(
		$.mobile.activePage.attr('id') != "page_call" &&
		$.mobile.activePage.attr('id') != "page_user" &&
		$.mobile.activePage.attr('id') != "page_tag" 
	) 
	{
	 	user_page_origin = $.mobile.activePage.attr('id');
	}
	$.mobile.changePage($("#page_user"),"slide");
	
	// set page
	$("#page_user_name").empty().append("@" +name);
	$("#user_chat_name").val(name);
	$("#user_chat_ip").val(ip);
	$("#page_user_msgs").empty();
	$("#page_user_files").empty();
	$("#page_user_queue").empty();
	$("#user_chat_msg").val("");
	
	// load messages
	get_user_msgs();
    // get info from remote user
    load_remote_userinfo(name, ip, id);
}

function load_remote_userinfo(name, ip, id)
{
	$("#page_user_files").empty().append("<p class=\"user-file_loading\"><img src=\"images/i_loading_15.gif\"/></p>");
	var path = "http://" +ip +":8081/pub_info.json";
    $.jsonp({
      url: path,
      callback: "abc",
      data: {},
      dataType: "jsonp",
	  timeout: 5000,
      success: function(data) {
			$("#page_user_files").empty();
			var nofiles = true;
			$.each(data.files, function(i,item)
			{
				nofiles = false;
				var file = "<div class=\"file\">";
				file    += file_button_schedule(item.hash, item.suffix, item.size, item.description, name, ip);
				file    += "<div class=\"filename\">" +format_msg_txt(item.description) +"</div>";
				file    += "<div class=\"filemeta\"><span class=\"suffix\">" +item.suffix +"</span> <span class=\"size\">" +file_filesize(item.size) +"</span> ";
				file    += '<abbr class="timeago" title="' +item.time +'">' +time2str(item.time) +'</abbr>';
				file    += "</div>";
				file    += "</div>";
				var myfile = $("#page_user_files").append(file);
				myfile.trigger('create');
			});
			
			if(nofiles)
			{
				$("#page_user_files").empty().append("<p class=\"user-file_info\">" +$.i18n._("%s has no shared files", [name]) +"</p>");
			}
      },
      error: function(d,msg) {
          if($("#user_chat_ip").val() == ip)
          {
			  // show info
			  var myfile = $("#page_user_files").empty().append("<p class=\"user-file_info\">" +$.i18n._("User not reachable") +"<br/><br/> " +"<a onclick=\"javascript:load_remote_userinfo('" +name +"', '" +ip +"')\" data-role=\"button\" data-iconpos=\"notext\" data-icon=\"refresh\" style=\"margin:0 auto !important;\">&nbsp;</a>" +"</p>");
			  myfile.trigger('create');
          }
      }
    });
}

function show_tag(tag)
{
	tag_last_id = 0;
	tag_name = tag;
	
	if(
		$.mobile.activePage.attr('id') != "page_call" &&
		$.mobile.activePage.attr('id') != "page_user" &&
		$.mobile.activePage.attr('id') != "page_tag" 
	) 
	{
	 	user_page_origin = $.mobile.activePage.attr('id');
	}
	// open page
	$.mobile.changePage($("#page_tag"),"slide");
	// set page
	$("#page_tag_name").empty().append(tag);
	$("#page_tag_msgs").empty();
	// load messages
	get_tag_msgs();
	
	return true;
}

// invoked before a new page will load
function onPageBeforeChange(event, data)
{
	removeIFrame();
}

// invoked after a new page was loaded
function onPageChange(event, data) 
{
	// create iFrame
	createIFrame();
	
	// actualize footer
	update_footer();
	
	// send page id to app
	update_pageid();
}    

function update_pageid()
{
	if(qaul_initialized && $.mobile.activePage.attr("id") != "page_loading" && !/page_config*/.test($.mobile.activePage.attr("id")))
	{
		$.get('setpagename?p=' +$.mobile.activePage.attr("id") +'&e=1');
	}
}

function quit_qaul()
{
	$.mobile.changePage('#page_goodbye');
	$.ajax({
		url: "quit",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data){
		}
	}).error(function(){
		$.mobile.changePage('#page_pref');
	});
}

function qaul_openurl(url)
{
	$.post(
		"setopenurl.json",
		{"url": url, "e":1}
	).error(function(){
		alert("error qaul_openurl");
	});
}

// ======================================================
// VoIP
// ------------------------------------------------------
var call_button_accept = '<a href="javascript:call_accept();" data-role="button" class="call_button_accept">&nbsp;</a>';
var call_button_reject = '<a href="javascript:call_end();" data-role="button" class="call_button_reject">&nbsp;</a>';
var call_button_end = '<a href="javascript:call_end();" data-role="button" class="call_button_end">&nbsp;</a>';

function call_start()
{
	var name = $("#user_chat_name").val();
	var ip = $("#user_chat_ip").val();
	// change page
	$("#call_info").html($.i18n._("Connecting") +'<br/><img src="images/i_loading_15.gif"/>');
	call_show_page(name);
	call_setButtonEnd();
	// start call
	var path = 'call_start?ip=' +ip +'&e=1';
	$.ajax({
		url: path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data){
		}
	}).error(function(){
		$("#call_buttons").empty();
		call_goback();
	});
}

function call_end()
{
	$.ajax({
		  url: "call_end",
		  cache: false, // needed for IE
		  dataType: "json",
		  success: function(data){
			  $("#call_buttons").empty();
			  call_goback();
		  }
	});
}

function call_accept()
{
	$.ajax({
		  url: "call_accept",
		  cache: false, // needed for IE
		  dataType: "json",
		  success: function(data){
			  // nothing to be done here
		  }
	});
}

function call_show_page(name)
{
	 if($.mobile.activePage.attr('id') != "page_call") 
	 {
	 	call_page_origin = $.mobile.activePage.attr('id');
	 	$.mobile.changePage($("#page_call"));
	 }
	 // set name
	 $("#call_user").text(name);
	 callchecktimer();
}

function call_schedule_end(reason)
{
	// remove buttons
	var mybutton = $("#call_buttons").empty().append('<a href="javascript:call_goback();" data-icon="arrow-l" data-inline="true" data-role="button">' +$.i18n._("Back") +'</a>');
	mybutton.trigger('create');
	// set text
	$("#call_info").text(reason);
	// set time before going back
	setTimeout(function(){call_goback();},2000);
}

function call_setButtonEnd()
{
	var mybutton = $("#call_buttons").empty().append(call_button_end);
	mybutton.trigger('create');
}

function call_setButtonsIncoming()
{
	var mybutton = $("#call_buttons").empty().append(call_button_accept +call_button_reject);
	mybutton.trigger('create');
}

function call_goback()
{
	$.mobile.changePage($("#" +call_page_origin));
}

function call_setRinging()
{
	$("#call_info").html($.i18n._("Ringing") +'<br/><img src="images/i_loading_15.gif"/>');
	call_setButtonEnd();
}

function call_setCalling()
{
	$("#call_info").text($.i18n._("Is calling"));
	call_setButtonsIncoming();
}

function call_setConnecting()
{
	$("#call_info").html($.i18n._("Establishing connection") +'<br/><img src="images/i_loading_15.gif"/>');
	call_setButtonEnd();
}

function call_setConnected()
{
	$("#call_info").html($.i18n._("Connected") +'<br/><img src="images/i_call_32.png"/>');
	call_setButtonEnd();
}

function call_setEnded(code)
{
	var reason;
	if(code == 486)
		reason = $.i18n._("Is busy");
	else if(code == 487)
		reason = $.i18n._("Call ended");
	else
		reason = $.i18n._("Not reachable");
		
	call_schedule_end(reason);
}

var callchecktimer = function()
{
	// check call status
	$.ajax({
		url:   "call_event",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			if($.mobile.activePage.attr('id')=="page_call")
			{
				if(data.event == 0); // nothing new happend
				else if(data.event == 1)
				{
					call_setRinging();
				}
				else if(data.event == 2)
				{
					call_setCalling();
				}
				else if(data.event == 3)
				{
					call_setConnecting();
				}
				else if(data.event == 4)
				{
					call_setConnected();
				}
				else if(data.event == 5)
				{
					call_setEnded(data.code);
				}
				else return;
				// rescheduled
				setTimeout(function(){callchecktimer();},500);
			}
		} 
	}).error(function(){
			if($.mobile.activePage.attr('id')=="page_file_add")
			{
				setTimeout(function(){callchecktimer();},500);
			}
	});		
}

// ======================================================
// Chat
// ------------------------------------------------------

function format_msg_txt(msg)
{
	// @user
	msg = msg.replace(/(^|\s)(@[^\s]+)/g,"$1<span class=\"user\">$2</span>");
	// #tags
	msg = msg.replace(/(^|\s)(#[^\s]+)/g,"$1<a href=\"#page_tag\" onClick=\"javascript:show_tag('$2');\" class=\"tag\">$2</a>");
	// files
	//msg = msg.replace(/(^|\s)([a-zA-Z0-9]{40}\.([a-zA-Z0-9]{1,5}))/g,"$1<a href=\"\" class=\"file\">[$3]</a>");
	// emails
	msg = msg.replace(/[^\s]+@[^\s]{2,}\.[^\s]{2,}/g,"<a href=\"mailto:$&\" class=\"mail\">$&</a>");	
	// links
	msg = msg.replace(/http:\/\/([^\s]{5,})/g,"<a href=\"$&\" target=\"_blank\" class=\"link\">$1</a>");
	// emoticons
	msg = msg.replace(/(^|\s):-?\)/g,"$1<img src=\"images/emo_smile.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s):-?\(/g,"$1<img src=\"images/emo_sad.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s):-?D/g,"$1<img src=\"images/emo_lough.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s):-?o/g,"$1<img src=\"images/emo_astonishing.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s);-?\)/g,"$1<img src=\"images/emo_joking.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s):-?P/g,"$1<img src=\"images/emo_tongue.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s)8-?\)/g,"$1<img src=\"images/emo_cool.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s):-?\//g,"$1<img src=\"images/emo_embarassed.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s):-?\|/g,"$1<img src=\"images/emo_confused.png\" class=\"emo\" />");
	msg = msg.replace(/(^|\s):-?@/g,"$1<img src=\"images/emo_shouting.png\" class=\"emo\" />");
	return msg;
}

function format_msg_file(msg, desc, name, ip)
{
	// files
	desc = desc.replace(/(^|\s)([a-zA-Z0-9]{40}\.[a-zA-Z0-9]{1,5})/g,"$1");
	desc = $.trim(desc);
	// with button
	var button = "";
	msg = msg.replace(/(^|\s)([a-zA-Z0-9]{40})\.([a-zA-Z0-9]{1,5})/g, function(a,b,c,d){
				button = file_button_schedule( c, d, 0, desc, name, ip);
				return "<span class=\"suffix\">" +d +"</span> <span class=\"size\">" +b +"</span>";
		});
	return {"msg":msg,"button":button};
}

function format_msg_voip(item)
{
	var button;
	var msg;
	if(item.type == 3)
	{
		button = '<div class="msg_voip"><img src="images/i_call_in_64.png" /></div>';
		msg = $.i18n._("Incoming call from %s", [format_msg_userlink(item.name, item.ip, item.id)]);
	}
	else
	{
		button = '<div class="msg_voip"><img src="images/i_call_out_64.png" /></div>';
		msg = $.i18n._("You called %s", [format_msg_userlink(item.name, item.ip, item.id)]);
	}
		
	return {"msg":msg,"button":button};
}

function format_msg_userlink(name, ip, id)
{
	return '<a href="#page_user" onClick="javascript:show_user(\'' +name +'\',\'' +ip +'\',\'' +id 
					+'\');">' +name +'</a>';
}

function format_msg(item)
{
	// format message
	var formated;
	if(item.type == 3 || item.type == 13)
		formated = format_msg_voip(item);
	else
		formated = format_msg_file(format_msg_txt(item.msg), item.msg, item.name, item.ip);
	
	// create html
	var msg = '<div id="msg_' +item.id +'" class="msg msg_' +item.type  +'">';
	msg += formated.button;
	msg += '<div class="msg_time"><abbr class="timeago" id="abbr_msg_' +item.id +'" title="' +item.time +'">' +time2str(item.time) +'</abbr></div>';
	// from
	if(item.type == 3 || item.type == 13)
		;
	else if(item.type < 10)
		msg += '<div class="sender">' +format_msg_userlink(item.name, item.ip, item.id) +'</div>';
	else 
		msg += '<div class="sender">' +user_name +'</div>';

	// message
	msg += '<div class="message">' +formated.msg +'</div>';
	msg += '</div>';

	return msg;
}

function insert_msg(insert, item, inverse)
{
	var new_item = format_msg(item);
	if(!qaul_config.is_mobile) 
		new_item = $(new_item).hide();
	var myitem;
	if(inverse) 
		myitem = insert.append(new_item);
	else 
		myitem = insert.prepend(new_item);
	myitem.trigger('create');
	if(!qaul_config.is_mobile)
	{
		new_item.slideDown().fadeIn('slow');
	}
	insert.children("div.msg:gt(" +qaul_config.msg_max +")").remove();
}

function send_msg()
{
	$.post(
			"sendmsg",
			{ "t": 11, "m": msg.val(), "n": user_name, "e":1},
			function(){
				//insert_msg(chat, {id:0,type:11,name:user_name,msg:msg.val(),time:isoDateString(new Date())});
				msg.val('');
				get_msgs();
			}
		).error(function(){
			// show alert
			$.mobile.changePage($("#page_dialog"),{role:"dialog"});
		});
};

function send_tag_msg()
{
	$.post(
			"sendmsg",
			{"t": 11, "m": $("#tag_chat_msg").val(), "n": user_name, "e":1},
			function(){
				var mymsg = $("#tag_chat_msg").val();
				if(mymsg.indexOf(tag_name)==-1)
					insert_msg($("#page_tag_msgs"), {id:0,type:11,name:user_name,msg:$("#tag_chat_msg").val(),time:isoDateString(new Date())});
				$("#tag_chat_msg").val('');
				get_tag_msgs();
			}
		).error(function(){
			// show alert
			$.mobile.changePage($("#page_dialog"),{role:"dialog"});
		});
}

function send_direct_msg()
{
    // set loading info
    //$.mobile.pageLoading();
    $("#page_user_queue").empty().append("<p class=\"user_msg_loading\"><img src=\"images/i_loading_15.gif\"/></p>");

    // send data to remote user
    $.jsonp({
      url: "http://" +$("#user_chat_ip").val() +":8081/pub_msg",
      callback: "abc",
      data: {
			"n": user_name,
			"m": $("#user_chat_msg").val(),
			"e": 1
      },
      dataType: "jsonp",
	  timeout: 4000,
      success: function(userProfile) {
          	// clear loading info
          	$("#page_user_queue").empty();
          	
          	// send message to db
			$.post(
					'sendmsg',
					{"t": 12, "m": $("#user_chat_msg").val(), "n": user_name, "e":1},
					function(){
						// insert sent message
						insert_msg($("#page_user_msgs"), {id:0,type:12,name:user_name,to:$("#user_chat_name").val(),msg:$("#user_chat_msg").val(),time:isoDateString(new Date())});
						// clear message input
			            $("#user_chat_msg").val('');
			            // get message from DB
			            get_user_msgs();
					}
			);
      },
      error: function(d,msg) {
      		$("#page_user_queue").empty().append("<p class=\"user_msg_failed\">" +$.i18n._("User not reachable") +"</p>");
      }
    });
}

function get_msgs()
{
	var path = "getmsgs.json?t=1&id=" +msg_last_id +"&e=1";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			$.each(data.messages, function(i,item){
				if(item.id > msg_last_id)
					msg_last_id = item.id;
				if($('#msg_' +item.id).length == 0)
					insert_msg(chat, item);
			})
		} 
	});
}

function get_user_msgs()
{
	var path = 'getmsgs.json?t=5&id=' +user_last_id +'&v=' +encodeURIComponent($("#user_chat_name").val()) +'&e=1';
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			var inverse = false;
			$.each(data.messages, function(i,item){
					if(item.id > user_last_id)
						user_last_id = item.id;
					insert_msg($("#page_user_msgs"), item, inverse);
			})
		} 
	});
}

function get_tag_msgs()
{
	var path = 'getmsgs.json?t=6&id=' +tag_last_id +'&v=' +encodeURIComponent(tag_name) +'&e=1';
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			var inverse = false;
			$.each(data.messages, function(i,item){
					if(item.id > tag_last_id)
						tag_last_id = item.id;
					insert_msg($("#page_tag_msgs"), item, inverse);
			})
		} 
	});
}

function time2str(timestr)
{
	return timestr.substr(11,5);
}

function timestamp2str(timestamp)
{
	var date = new Date(timestamp *1000);
	return date.getHours() +":" +date.getMinutes();
}

var updatetimer=function()
{
	if($.mobile.activePage.attr("id")=="page_chat")
	{
		get_msgs();
	}
	else if($.mobile.activePage.attr("id")=="page_users")
	{
		get_users();
	}
	else if($.mobile.activePage.attr("id")=="page_user")
	{
		get_user_msgs();
	}
	else if($.mobile.activePage.attr("id")=="page_tag")
	{
		get_tag_msgs();
	}
	else if($.mobile.activePage.attr("id")=="page_file")
	{
		file_update();
	}
	
	setTimeout(function(){updatetimer();},3000);
};

// ======================================================
// files
// ------------------------------------------------------
function show_page_file()
{
	file_update();
	$.mobile.changePage($("#page_file"));
}

function send_file_add()
{
	// check if file was selected
	if($("#file_add_path").val() == "")
	{
		// open file select again
		open_filepicker();
		return;
	}
	
	// validate entries
	var advertise = ($("#file_add_advertise").attr('checked'))? 1 : 0;
	
	// send it to webserver
	$.ajax({
			type:'POST',
			url:"file_add.json",
			data:{"p": $("#file_add_path").val(), "m": $("#file_add_msg").val(), "a": advertise, "e": 1},
			cache: false, // needed for IE
			success: function(data){
				// TODO: insert message
				// configure message
				//var message = data.hash +"." +data.suffix +" " +$("#file_add_msg").val();
				//insert_msg(chat, {id:0,type:11,name:user_name,msg:message});
				// cleanup
				$("#file_add_msg").val('');
				$("#file_add_advertise").attr('checked',true).checkboxradio("refresh");
				$("#file_add_path").val('');
				$("#file_add_filename").empty();
				$("#file_add_addbutton .ui-btn-text").text("choose file");
				$("#file_add_addbutton .ui-icon").addClass("ui-icon-plus").removeClass("ui-icon-refresh");
				
				// go to file page
				show_page_file();
			},
			dataType:"json"
	}).error(function(){
			show_page_file();
	});
}

function show_addfile_page()
{
	// reset message field
	$("#file_add_msg").val('');
	// show page
	$.mobile.changePage($("#page_file_add"));
	open_filepicker();
}

function open_filepicker()
{
	// send message / open socket to show filepicker
	var path = "file_pick.json";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			// set timer to check if a file was picked
			setTimeout(function(){filepickertimer();},1000);
		} 
	});
}

function open_file(hash)
{
	// send message / open socket to show filepicker
	var path = "file_open.json?f=" +hash;
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			// success
		} 
	});
}

function file_delete(hash)
{
	// delete file by id
	var path = "file_delete.json?hash=" +hash;
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			// reload files
			file_update();
		} 
	});
}

function file_advertise(hash, suffix, size, description)
{
	// open chat with filled in message
	msg.val(hash +"." +suffix +" " +description);
	$.mobile.changePage($("#page_chat"),{});
}

function file_suffix2icon(suffix)
{
	return "<img src=\"images/" +file_suffix2filetype(suffix) +"_64.png\" class=\"fileicon64\">";
}

function file_suffix2filetype(suffix)
{
	var type;
	if(suffix.match(/^((jpe?g)|(png)|(gif)|(bmp)|(tiff?)|(raw)|(svg))$/i)) type = "f_img";
	else if(suffix.match(/^((mov)|(3gpp?)|(avi)|(mpg)|(mp4)|(m4v)|(wmv)|(flv))$/i)) type = "f_mov";
	else if(suffix.match(/^((mp3)|(wav)|(ogg)|(aiff?)|(m4a)|(m4p))$/i)) type = "f_sound";
	else if(suffix.match(/^((pdf)|(ps))$/i)) type = "f_pdf";
	else if(suffix.match(/^((txt)|(rtf)|(html?)|(docx?)|(xls)|(xml)|(ppt)|(odt))$/i)) type = "f_txt";
	else if(suffix.match(/^((zip)|(tar)|(gz))$/i)) type = "f_zip";
	else type = "f_file";
	return type;
}

function file_button_schedule(hash, suffix, size, description, name, ip)
{
	var button = "";
	if(file_check(hash, suffix)) button += "<div class=\"msg_filebutton " +hash +suffix +" " +file_suffix2filetype(suffix) +"\"><a href=\"#page_file\" class=\"" +file_suffix2filetype(suffix) +"\"><img src=\"images/f_success_64.png\"/></a></div>";
	else button += "<div class=\"msg_filebutton " +hash +suffix +"\"><a href=\"#\" onClick=\"javascript:file_schedule('" +hash +"','" +suffix +"','" +description +"','" +size +"','" +ip +"','" +name +"')\" class=\"" +file_suffix2filetype(suffix) +"\"><img src=\"images/f_add_64.png\"/></a></div>";
	return button;
}

function file_button_deactivate(hash, suffix)
{
	var button = "<img src=\"images/f_success_64.png\"/>";
	var a = $("div." +hash +suffix +" a");
	a.prop("onclick", null);
	a.prop("href", "#page_file");
	a.empty().append(button);
	//var button = "<a href=\"#page_file\" class=\"filebutton\">file scheduled</a>";
	//$("div." +hash +suffix).empty().append(button).trigger('create');
}

function file_check(hash, suffix)
{
	var i;
	for(i=0; i < qaulfiles.length; i++)
	{
		if(qaulfiles[i].status > -2 && qaulfiles[i].suffix == suffix && qaulfiles[i].hash == hash) 
			return true;
	}
	return false;
}

function file_update()
{
	var path = "file_list.json?r=0&e=1";
	var files = $("#page_file_list");
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			$.each(data.files, function(i,item){
				file_update_check(item);
			});
		} 
	}).error(function(){
		// fail silently
	});
}

// update file list & entries
function file_update_check(item)
{
	var exists = false;
	var i;
	for(i=0; i < qaulfiles.length; i++)
	{
		if(qaulfiles[i].hash == item.hash)
		{
			// update entry
			exists = true;
			// delete entry
			if(item.status == QAUL_FILESTATUS_DELETED) 
			{
				$("#file_" +item.hash).remove();
				qaulfiles.splice(i,1);
			}
			// download failed
			else if(item.status == QAUL_FILESTATUS_ERROR)
			{
				$("#file_" +item.hash).removeClass("scheduled downloading").addClass("failed");
				$("#file_" +item.hash +" .fileicon64").attr("src","images/f_failed_64.png");
				$("#file_bar_" +item.hash).remove();
				qaulfiles[i] = item;
			}
			// update progress bar
			else if(item.status == QAUL_FILESTATUS_DOWNLOADING)
			{
				$("#file_bar_" +item.hash).progressBar(item.downloaded);
				$("#file_" +item.hash +" span.size").text(file_filesize(item.size));
				qaulfiles[i] = item;
			}
			// file sucessfully downloaded
			else if(item.status == QAUL_FILESTATUS_DOWNLOADED)
			{
				if(qaulfiles[i].status <= QAUL_FILESTATUS_DOWNLOADING)
				{
					$("#file_" +item.hash).removeClass("scheduled downloading");
					$("#file_bar_" +item.hash).progressBar(100);
					// add open file link
					$("#file_" +item.hash +" img.fileicon64").wrap("<a href=\"#\" onClick=\"javascript:open_file('" +item.hash +"')\"></a>");
					// add readvertise button
					var button = "<a href=\"#\" onClick=\"javascript:file_advertise('" +item.hash +"','" +item.suffix +"','" +item.size +"','" +item.description +"')\" class=\"filebutton\"><img src=\"images/b_advertise.png\" alt=\"advertise\" /></a>";
					$("#file_" +item.hash +" a.filebutton").after(button);
				}
				qaulfiles[i] = item;
			}
			else if(item.status == QAUL_FILESTATUS_NEW)
			{
				if(item.status != qaulfiles[i].status)
				{
					$("#file_" +item.hash).remove();
					qaulfiles.splice(i,1);
					exists = false;
				}
				else
					qaulfiles[i] = item;
			}
			break;
		}
	}
	
	// add file if not existing
	if(!exists && item.status != QAUL_FILESTATUS_DELETED)
	{
		qaulfiles.push(item);
		var htmlitem = file_create_html(item);
		var myitem = $("#page_file_list").prepend(htmlitem);
		myitem.trigger('create');
		
		// downloader bar
		var percent = 0;
		if(item.status == QAUL_FILESTATUS_DOWNLOADING)
			percent = item.downloaded;
		myitem.find("#file_bar_" +item.hash).progressBar(percent,{barImage:'images/progressbg_black.gif'});
		
		// deactivate schedule buttons
		file_button_deactivate(item.hash, item.suffix);
	}
	
	// rotate loader image
	if(item.status == QAUL_FILESTATUS_COPYING)
		$("img.loadericon64:visible").addClass("rotate");
}

function file_create_html(item)
{
	var fileclass = "";
	if(item.status == QAUL_FILESTATUS_MYFILE)
		fileclass = "file_myfile";
	else if(item.status == QAUL_FILESTATUS_COPYING)
		fileclass = "file_copying file_myfile";
	else if(item.status < QAUL_FILESTATUS_NEW)
		fileclass = "file_failed";
	
	var percent = 0;
	if(item.status == QAUL_FILESTATUS_DOWNLOADING)
		percent = item.downloaded;
	
	var file = "<div class=\"file " +fileclass +"\" id=\"file_" +item.hash +"\">";
	if(item.status >= QAUL_FILESTATUS_DOWNLOADED) 
		file += "<a href=\"#\" onClick=\"javascript:open_file('" +item.hash +"')\">";
	
	if(item.status == QAUL_FILESTATUS_COPYING) 
		file += "<img src=\"images/ajax-loader.png\" class=\"loadericon64\">";
	else if(item.status <= QAUL_FILESTATUS_ERROR) 
		file += "<img src=\"images/f_failed_64.png\" class=\"fileicon64\">";
	else
		file += file_suffix2icon(item.suffix);
	
	if(item.status >= QAUL_FILESTATUS_DOWNLOADED)
		file += "</a>";
	
	if(item.status != QAUL_FILESTATUS_COPYING)
		file += "<a href=\"#\" onClick=\"javascript:file_delete('" +item.hash +"')\" class=\"filebutton\"><img src=\"images/b_delete.png\" alt=\"delete\" /></a>";
	
	if(item.status >= QAUL_FILESTATUS_DOWNLOADED) 
		file += "<a href=\"#\" onClick=\"javascript:file_advertise('" +item.hash +"','" +item.suffix +"','" +item.size +"','" +item.description +"')\" class=\"filebutton\"><img src=\"images/b_advertise.png\" alt=\"advertise\" /></a>";
	
	file     += "<div class=\"filename\">" +format_msg_txt(item.description) +"</div>";
	if(item.status >= QAUL_FILESTATUS_NEW && item.status <= QAUL_FILESTATUS_DOWNLOADING)
		file += "<div class=\"fileprogress\"><span class=\"progressBar\" id=\"file_bar_" +item.hash +"\">" +percent +"%</span></div>";
	file     += "<div class=\"filemeta\"><span class=\"suffix\">" +item.suffix +"</span> <span class=\"size\">" +file_filesize(item.size) +"</span> " ;
	file     += '<abbr class="timeago" id="abbr_msg_' +item.hash +'" title="' +item.time +'">' +time2str(item.time) +'</abbr>';
	file     += "</div>";
	file     += "</div>";
	return file;
}

var filepickertimer=function()
{
	// check if file was selected
	var path = "file_pickcheck.json";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			if($.mobile.activePage.attr('id')=="page_file_add")
			{
				if(data.picked == 0); // picking was canceld
				else if(data.picked == 2)
				{
					// display picked file
					$("#file_add_path").val(data.path);
					$("#file_add_filename").text(data.name);
					
					// take filename as message if message is empty
					if($("#file_add_msg").val()=="")
					{
						var name = data.name;
						name = name.replace(/\.[a-zA-Z0-9]+$/g,"");
						$("#file_add_msg").val(name.replace(/[._-]+/g," "));
					}
				}
				else setTimeout(function(){filepickertimer();},400);
			}
		} 
	}).error(function(){
			if($.mobile.activePage.attr('id')=="page_file_add")
			{
				// show alert
				//alert("error filepickertimer");
				setTimeout(function(){filepickertimer();},400);
			}
	});	
};

var loadingtimer=function()
{
	// check if file was selected
	var path = "loading.json";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			if($.mobile.activePage.attr('id')=="page_loading")
			{
				if(data.change == 1) // change page
				{
					// initialize chat
					if(data.page == "#page_chat" && !chat_initialized) 
					{
						init_chat();
						// load favorites
						init_favorites();
					}
					// display page
					$.mobile.changePage($(data.page));
				}
				else 
					setTimeout(function(){loadingtimer();},500);
			}
		} 
	}).error(function(){
			if($.mobile.activePage.attr('id')=="page_loading")
			{
				setTimeout(function(){loadingtimer();},500);
			}
	});	
};

function file_schedule(hash, suffix, description, size, ip, name)
{
	// send message / open socket to show filepicker
	var path = "file_schedule.json";
	$.ajax({
		type: 'POST',
		url:   path,
		data:  {"hash": hash, "suffix": suffix, "description": description, "size": size, "ip": ip, "name": name, "e":1},
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			// go to file page
			show_page_file();
		} 
	}).error(function(){
		// show alert
		alert("error scheduling file");
	});
	
}

function file_filesize(size)
{
	var str = "";
	if(size > 10000000000) str += (Math.round(size/1000000000)) +"GB";
	else if(size > 1000000000) str += (Math.round(size/100000000)/10) +"GB";
	else if(size > 10000000) str += (Math.round(size/1000000)) +"MB";
	else if(size > 1000000) str += (Math.round(size/100000)/10) +"MB";
	else if(size > 10000) str += (Math.round(size/1000)) +"KB";
	else if(size > 1000) str += (Math.round(size/100)/10) +"KB";
	else if(size > 0) str += "1KB";
	return str;
}

function isoDateString(d)
{
	function pad(n){return n<10 ? '0'+n : n};
	
	return d.getFullYear()+'-'
      + pad(d.getMonth()+1)+'-'
      + pad(d.getDate())+'T'
      + pad(d.getHours())+':'
      + pad(d.getMinutes())+':'
      + pad(d.getSeconds())+'Z';
}

// ======================================================
// configuration
// ------------------------------------------------------

function send_locale()
{
	// send locale
	$.post(
			'setlocale',
			{"l": $("input[name='l']:checked").val(), "e":1},
			function(data){
				if(chat_initialized)
				{
					$.mobile.changePage($("#page_restart"));
				}
				else
				{
					// forward to loading
					$.mobile.changePage($("#page_loading"));
					// set timer to check which page to load
					setTimeout(function(){loadingtimer();},1000);
					// update configuration
					init_config();
				}
		});
};

function send_name()
{
	// send user name
	$.post(
			'setname',
			{"n": $("#name_name").val(), "e":1},
			function(data){
				// update username
				set_username($("#name_name").val());
				// forward to loading
				$.mobile.changePage($("#page_loading"));
				// set timer to check which page to load
				setTimeout(function(){loadingtimer();},1000);
		});
};

// interface configuration
function config_interface_show()
{
	// show loading page
	$.mobile.changePage($("#page_loading"));
	
	// request interface configuration
	$.ajax({
		url:   "config_interface_loading",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			$.mobile.changePage($("#page_loading"));
			setTimeout(function(){loadingtimer();},500);
		}
	}).error(function(){
		$.mobile.changePage($("#page_pref"));
	});
	
	return true;
}

function config_interface_load_data()
{
	// load configuration
	$.ajax({
		url:   "config_interface_get",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			config_interface_data_loaded(data);
		}
	});
}

function config_interface_data_loaded(data)
{
	// toggle flipswitch
	if(data.manual == 1)
	{
		$("#interface_select_auto").val('1').flipswitch('refresh');
		$(".c_interface_manual").show();
	}
	else
	{
		$("#interface_select_auto").val('0').flipswitch('refresh');
		$(".c_interface_manual").hide();
	}
	
	// populate interfaces
	var myhtml = "<fieldset data-role=\"controlgroup\" id=\"interface_selection\">";
	$.each(data.interfaces, function(i,item)
	{
		myhtml += "<input type=\"radio\" name=\"if\" value=\"" +item.name +"\" id=\"if_" +item.name +"\" ";
		if(item.name == data.selected)
			myhtml += "checked=\"checked\" ";
		myhtml += "class=\"interface_select_checkbox\" />";
		myhtml += "<label for=\"if_" +item.name +"\">" +item.ui_name +"</label>";
	});
	myhtml += "</fieldset>";
	$("#c_interface_manual").empty().append(myhtml).trigger("create");
}

// show/hide interfaces when toggle flipswitch button
function config_interface_toggle()
{
	if($("#interface_select_auto").val() == 1)
		$(".c_interface_manual").show();
	else
		$(".c_interface_manual").hide();
}

function config_interface_send()
{
	// check which interfaces to send
	var interfaces = "";
	$.each($(".interface_select_checkbox:checked"), function(i,item){
		if(interfaces.length > 0)
			interfaces += " ";
		interfaces += $(item).val();
	});
	
	// send configured interface
	$.post(
			'config_interface_set',
			{"m": $("#interface_select_auto").val(), "if": interfaces, "e":1},
			function(data){
				// forward to restart page
				$.mobile.changePage($("#page_restart"));
		});
};

// show network configuration page
function config_network_show()
{
	$.mobile.changePage($("#page_config_network"));
	$("#c_network_config").hide();
	$("#c_network_info").hide();
	config_network_get();
}

// get current network configuration
function config_network_get()
{
	// request interface configuration
	$.ajax({
		url:   "config_network_get",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data){
			config_network_profile.active = data.profile;
			config_network_profile.data = data;
			// todo: does this send a refresh event?
			$("#c_network_profile").val(data.profile).selectmenu("refresh");
			config_network_template(data);
		}
	});
}

// network community profiles
function config_network_change()
{
	$("#c_network_config").hide();
	$("#c_network_info").hide();
	var profile = $("#c_network_profile").val();
	
	// check if it is the active profile
	if(config_network_profile.active == profile)
		config_network_get();
	else
	{
		// check previously saved values
		var path = "config_network_profile?p=" +profile +"&e=1";
		$.ajax({
			url: path,
			cache: false, // needed for IE
			dataType: "json",
			success: function(data){
				config_network_profile.data = data;
				config_network_template();
			}
		});
	}
}

function config_network_template()
{
	$.ajax({
		url: "community_templates/" + $("#c_network_profile").val(),
		cache: false, // needed for IE
		dataType: "json",
		success: function(data){
			config_network_profile.template = data;
			config_network_data();
		}
	});
}

function config_network_data()
{
	var data = config_network_profile.data;
	var template = config_network_profile.template;
	
	if($("#c_network_profile").val() == "qaul")
		$("#c_network_info").show();
	
	// fill in values
	if(data.available)
	{
		$("#c_network_config_ip").val(data.ip);
		$("#c_network_config_mask").val(data.mask);
		$("#c_network_config_broadcast").val(data.broadcast);
		$("#c_network_config_channel").val(data.channel).selectmenu("refresh");
		$("#c_network_config_ssid").val(data.ssid);
		$("#c_network_config_bssid").val(data.bssid);
	}
	else
	{
		// take values from community template
		if(template.ip.generate == "true")
			$("#c_network_config_ip").val(config_network_IP(template.ip.pattern));
		else
			$("#c_network_config_ip").val(template.ip.value);
		
		$("#c_network_config_mask").val(template.mask.value);
		$("#c_network_config_broadcast").val(template.broadcast.value);
		$("#c_network_config_channel").val(template.channel.value).selectmenu("refresh");
		$("#c_network_config_ssid").val(template.ssid.value);
		$("#c_network_config_bssid").val(template.bssid.value);
	}
	
	// hide, show, readonly
	config_network_field("c_network_config_ip", template.ip.edit);
	config_network_field("c_network_config_mask", template.mask.edit);
	config_network_field("c_network_config_broadcast", template.broadcast.edit);
	config_network_field("c_network_config_channel", template.channel.edit);
	config_network_field("c_network_config_ssid", template.ssid.edit);
	config_network_field("c_network_config_bssid", template.bssid.edit);
	
	// add information
	var info = '<a href="javascript:qaul_openurl(' +template.profile.homepage +')">' +template.profile.homepage +'</a><br/>';
	info += template.profile.info;
	$("#c_network_config_info").empty().append(info);
	
	// show configuration
	if($("#c_network_profile").val() != "qaul")
		$("#c_network_config").show();
}

// hide, show, readonly fields
function config_network_field(field, edit)
{
	
	if(edit == "hidden")
		$("."+field).hide();
	else
	{
		$("."+field).show();
		if(edit == "readonly")
			$("#"+field).attr('readonly', 'readonly');
		else
			$("#"+field).removeAttr('readonly');
	}	
}

// create random IP
function config_network_IP(pattern)
{
	var ip;
	var pat = pattern.split("/");
	var ipx = pat[0].split(".");
	var net = pat[1];
	
	if(net == 8)
	{
		ip = ipx[0] +"." +ipx[1] +"." +ipx[2] +"." +randomIpv4Part();
	}
	else if(net == 16)
	{
		ip = ipx[0] +"." +ipx[1] +"." +randomIpv4Part() +"." +randomIpv4Part();
	}
	else if(net == 24)
	{
		ip = ipx[0] +"." +randomIpv4Part() +"." +randomIpv4Part() +"." +randomIpv4Part();
	}
	
	return ip;
}

function randomIpv4Part()
{
  return Math.floor(Math.random() * (254 - 1)) + 1;
}

// send form
function config_network_send()
{
	var values = {
		"profile": $("#c_network_profile").val(), 
		"ip": $("#c_network_config_ip").val(), 
		"mask": $("#c_network_config_mask").val(), 
		"broadcast": $("#c_network_config_broadcast").val(), 
		"channel": $("#c_network_config_channel").val(), 
		"ssid": $("#c_network_config_ssid").val(), 
		"bssid": $("#c_network_config_bssid").val(),
		"e":1
		};
	
	$.post(
		'config_network_set',
		values,
		function(data){
			$.mobile.changePage($("#page_pref"));
	});
}

// Internet sharing configuration
function config_internet_show()
{
	// show loading page
	$.mobile.changePage($("#page_loading"));
	
	// request interface configuration
	$.ajax({
		url:   "config_internet_loading",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			$.mobile.changePage($("#page_loading"));
			setTimeout(function(){loadingtimer();},500);
		}
	}).error(function(){
		$.mobile.changePage($("#page_pref"));
	});
}

function config_internet_load_data()
{
	// load configuration
	$.ajax({
		url:   "config_internet_get",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			config_internet_data_loaded(data);
		}
	});
}

function config_internet_data_loaded(data)
{
	// toggle flipswitch
	if(data.autodownload == 1)
	{
		$("#c_internet_share").val('1').flipswitch('refresh');
		$("#c_internet_interface").show();
	}
	else
	{
		$("#c_internet_share").val('0').flipswitch('refresh');
		$("#c_internet_interface").hide();
	}
	
	// populate interfaces
	var myhtml = "<fieldset data-role=\"controlgroup\" id=\"interface_selection\">";
	$.each(data.interfaces, function(i,item)
	{
		if(item.name != data.used)
		{
			myhtml += "<input type=\"radio\" name=\"if\" value=\"" +item.name +"\" id=\"if_" +item.name +"\" ";
			if(item.name == data.selected)
				myhtml += "checked=\"checked\" ";
			myhtml += "class=\"internet_select_checkbox\" />";
			myhtml += "<label for=\"if_" +item.name +"\">" +item.ui_name +"</label>";
		}
	});
	myhtml += "</fieldset>";
	$("#c_internet_interface").empty().append(myhtml).trigger("create");
}

function config_internet_send()
{
	// find selected interface 
	var interfaces = "";
	$.each($(".internet_select_checkbox:checked"), function(i,item){
		if(interfaces.length > 0)
			interfaces += " ";
		interfaces += $(item).val();
	});
	
	// if no interface was selected, take the first one
	if($("#c_internet_share").val() == 1 && interfaces == "")
	{
		$("#c_internet_popup").popup("open");
	}
	else
	{
		// send form
		$.post(
			'config_internet_set',
			{"share": $("#c_internet_share").val(), "if": interfaces, "e":1},
			function(data){
				$.mobile.changePage($("#page_pref"));
		});
	}
}

// show/hide configuration
function config_internet_toggle()
{
	if($("#c_internet_share").val() == 1)
		$("#c_internet_interface").show();
	else
		$("#c_internet_interface").hide();
}

// file sharing configuration
function config_files_show()
{
	// request interface configuration
	$.ajax({
		url: "config_files_get",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			$.mobile.changePage($("#page_config_files"));
			config_files_data(data);
		}
	}).error(function(){
			alert("config_files_get failed");
	});
}

function config_files_data(data)
{
	// toggle flipswitch
	if(data.autodownload == 1)
	{
		$("#c_files_autodownload_select").val('1').flipswitch('refresh');
		$("#c_files_autodownload").show();
	}
	else
	{
		$("#c_files_autodownload_select").val('0').flipswitch('refresh');
		$("#c_files_autodownload").hide();
	}
	
	// set values
	$("#c_download_space").val(data.space);
	$("#c_download_filesize").val(data.filesize);
}

function config_files_send()
{
	$.post(
		'config_files_set',
		{"download": $("#c_files_autodownload_select").val(), "space": $("#c_download_space").val(), "size": $("#c_download_filesize").val(), "e":1},
		function(data){
			$.mobile.changePage($("#page_pref"));
	});	
}

// show hide autodownload configuration
function config_files_auto_toggle()
{
	if($("#c_files_autodownload_select").val() == 1)
		$("#c_files_autodownload").show();
	else
		$("#c_files_autodownload").hide();
}


var eventstimer=function()
{
	// check if file was selected
	var path = "getevents.json";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			// check for new incoming calls
			if(data.call > 0)
			{
				// show call page
				$("#call_info").text("calling");
				call_show_page(data.callee);
				// put buttons
				call_setButtonsIncoming();
			}
			// check for unchecked files
			if(!(typeof data.files === 'undefined'))
			{
				if(data.files > 0)
					$(".i_filesharing .ui-li-count").text(data.files);
				else
					$(".i_filesharing .ui-li-count").empty();
			}
			// check for unchecked messages
			if(!(typeof data.m_priv === 'undefined'))
			{
				if(data.m_priv > 0)
					$(".i_chat .ui-li-count").text(data.m_priv);
				else
					$(".i_chat .ui-li-count").empty();
			}
			// set timer
			setTimeout(function(){eventstimer();},1000);
		} 
	}).error(function(){
			setTimeout(function(){eventstimer();},1000);
	});	
};

// ======================================================
// Users
// ------------------------------------------------------

function get_users()
{
	var path = "getusers.json?r=0&e=1";
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			users_append(data);
		} 
	});
};

function users_append(data)
{
	var items = [];
	$.each(data.users, function(i,item){
		if(item.add == 1) 
			user_append(item.name, item.ip, item.id, item.lq);
		else if(item.add >= 2)
			user_remove(item.name, item.ip, item.id, item.lq);
	});
	$("#users").listview("refresh"); // This line now updates the listview
}

function user_append(name, ip, id, conn)
{
	// check if it is a favorite
	var usr = $("#favorites #" +id);
	if(usr.length)
	{
		usr.data('connection', conn)
			.find("a.fav")
			.removeClass("offline")
			.attr("onclick","").unbind("click").trigger("refresh");
			
		usr.find("a.fav img.ui-li-icon")
			.attr("src","images/i_conn" +conn +"_13.png");
	}
	else
	{
		usr = $("#users #" +id);
		if(usr.length)
		{
			// update user connection strength
			usr.data('connection', conn);
				
			usr.find(".ui-btn-text a img.ui-li-icon")
				.attr("src","images/i_conn" +conn +"_13.png");
			
			// todo: update incoming and queued messages
		}
		else
		{
			if(name.indexOf("[WEB]") > -1)
				webuser = ' onclick="javascript:return false;" class="webuser"';
			else
				webuser = '';
			
			$("<li></li>")
				.prop('id',id)
				.data('connection', conn)
				.html('<a href="javascript:show_user(\'' +name +'\',\'' +ip +'\',\'' +id 
					+'\')"' +webuser +'>' +'<img src="images/i_conn' +conn +'_13.png" class="ui-li-icon ui-corner-none"/>' +name 
					//+'<span class="ui-li-count msg_in">↑4 ↓3</span>' 
					+'</a>'
					+'<a href="javascript:favorite_add(\'' +name +'\',\'' +ip +'\',\'' +id +'\');" data-icon="plus">add</a>'
					)
				.insertAfter($("#users_divider"));
				
			users.listview("refresh");
		}
    }
}

function user_remove(name, ip, id, conn)
{
	// check if favorite
	if($("#favorites #" +id).length)
	{
		$("#favorites #" +id)
			.data('connection', conn);
		
		$("#favorites #" +id +" a.fav")
			.addClass("offline")
			.click(function(){
				return false;
			})
			.find("img.ui-li-icon").attr("src","images/i_conn0_13.png");
	}
	// remove from list
	if($("#users #" +id).length)
	{
		$("#users #" +id).remove();
	}
}

function set_usercount(nodes, users)
{
	node_count = nodes;
	user_count = users;
	update_footer();
}
function update_footer()
{
	// TODO: write notification into footer
	//$("#" +$.mobile.activePage.attr("id") +" .i_users .ui-btn-text").text(user_count +" (" +node_count +")");
}

function user_changetofiles(active)
{
	$("#page_user_tab_files a.ui-btn-active").removeClass("ui-btn-active");
	$("#page_user_tab_chat").hide();
	$("#page_user_tab_files").show();
}

function user_changetochat(active)
{
	$("#page_user_tab_chat a.ui-btn-active").removeClass("ui-btn-active");
	$("#page_user_tab_files").hide();
	$("#page_user_tab_chat").show();
}

function user_goback()
{
	$.mobile.changePage($("#" +user_page_origin));
}

function favorites_append(data)
{
	var items = [];
	$.each(data.favorites, function(i,item){
		favorite_append(item.name, item.ip, item.id, 0, false);
	});
	if ($("#favorites").hasClass('ui-listview')) 
   		$("#favorites").listview('refresh'); // list view as initialized and gets refreshed
	else
	    $("#favorites").trigger('create');
}

function favorite_append(name, ip, id, conn, online)
{
	var href = ' href="javascript:show_user(\'' +name +'\',\'' +ip +'\',\'' +id +'\')" ';
	var attr = ' class="fav"';
	
	if(!online) 
		attr = ' onclick="javascript:return false;" class="offline fav"';
	else if(name.indexOf("[WEB]") > -1)
	{
		href = ' href="javascript: return false;"';
		attr = ' onclick="javascript:return false;" class="webuser fav"';
	}
	
	$("<li></li>")
		.prop('id',id)
		.data('connection', conn)
		.html('<a ' +href +attr +'>' 
					+'<img src="images/i_conn' +conn +'_13.png" class="ui-li-icon ui-corner-none"/>' +name 
					//+'<span class="ui-li-count msg_in">↑4 ↓3</span>' 
					+'</a>'
					+'<a href="javascript:favorite_del(\'' +name +'\',\'' +ip +'\',\'' +id 
					+'\');" data-icon="minus">remove</a>'
					)
		.appendTo($("#favorites"));
}

function favorite_add(name, ip, id)
{
	var usr = $("#users #" +id)
	
	// get connection
	var conn = usr.data('connection');
	// remove user
	usr.remove();
	
	var path = "fav_add.json";
	$.ajax({
		type:'POST',
		url: path,
		data:{"ip":ip,"name":name,"id":id,"e":1},
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			favorite_append(name, ip, id, conn, true);
			$("#favorites").listview('refresh');
		} 
	}).error(function(){
		user_append(name, ip, id, conn);
	});
}

function favorite_del(name, ip, id)
{
	var fav = $("#favorites #" +id);
	var conn = fav.data('connection');
	
	var online = true;
	if(fav.find("a.offline").length)
		online = false;
	fav.remove();
	var path = "fav_del.json";
	$.ajax({
		type:'POST',
		url: path,
		data:{"ip":ip,"id":id,"e":1},
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			if(online)
				user_append(name, ip, id, conn);
		} 
	}).error(function(){
		favorite_append(name, ip, id, conn, online);
		$("#favorites").listview('refresh');
	});
}

// ======================================================
// Browser specific: Chrome
// ------------------------------------------------------
function createIFrame()
{
	// UI bugfix for chrome
	if(is_chrome)
	{
		var myheight;
		var myposition;
		var activepage = $.mobile.activePage.attr("id");
		var myfooter = $("#" +activepage +" .ui-footer-fixed");
		if(myfooter.length > 0)
		{
			myheight = myfooter.height();
			myposition = myfooter.position();
			// create iframe
			$.mobile.activePage.append('<div id="bugfix_footer" style="width:100%;height:' +myheight +'px;top:' +myposition.top +'px;left:0px;"><iframe src="blank.html" id="bugfix_footer_iframe" style="width:100%;height:' +myheight +'px;"></iframe></div>');
		}
		var myheader = $("#" +activepage +" .ui-header-fixed");
		if(myheader.length > 0)
		{
			myheight = myheader.height();
			myposition = myheader.position();
			// create iframe
			$.mobile.activePage.append('<div id="bugfix_header" style="width:100%;height:' +myheight +'px;top:' +myposition.top +'px;left:0px;"><iframe src="blank.html" id="bugfix_header_iframe" style="width:100%;height:' +myheight +'px;"></iframe></div>');
		}
	}
}
function removeIFrame()
{
	// UI bugfix for chrome
	if(is_chrome)
	{
		$("#bugfix_footer").remove();
		$("#bugfix_header").remove();
	}	
}

//-----------------------------------------------------

$(init_start);
