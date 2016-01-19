/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

// ======================================================
// initialize
// ------------------------------------------------------

function init_start()
{
	// load files
	load_binaries();
	
	// make sure we are on the front page
	$.mobile.changePage('#page_join');
}

// ======================================================
// functions
// ------------------------------------------------------
function load_binaries()
{
	var path = "ext_binaries.json";
	var files = $("#binaries");
	$.ajax({
		url:   path,
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			add_binaries(data);
		} 
	}).error(function(){
		// show alert
		alert("error file_update()");
		//$.mobile.changePage($("#page_dialog"),{role:"dialog"});
	});
}

function add_binaries(data)
{
	$.each(data.files, function(i,item){
		add_binary(item);
	});
}
function add_binary(item)
{
	$("<li></li>")
		.html('<a href="files/' +item.hash +'.' +item.suffix +'" rel="external">' +item.description +' [' +file_filesize(item.size) +']</a>')
		.appendTo($("#binaries"));
	
	$("#binaries").listview("refresh");
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
	else str += "1KB";
	return str;
}

// ======================================================
//alert('survived');
$(init_start);
