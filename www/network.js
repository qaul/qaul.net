var links, nodes, width, height, force, svg, path, circle, text;

// load nodes
function init_network()
{
	$.ajax({
		url:   "gettopology.json",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			if(data.available)
				draw_network(data);
			else
			{
				// data is not yet available schedule receive for later
				setTimeout(function(){init_network();}, 500);
			}
		}
	});
}

function draw_network(data)
{
	links = data.links;
	nodes = data.nodes;
	
	// search nodes from links
	// calculate link quality
	links.forEach(function(link) {
		if(typeof nodes[link.source] !== "undefined")
		{
			nodes[link.source].ip = link.source;
			link.source = nodes[link.source];
		}
		else 
			nodes[link.source] = {name: link.source, ip: link.source, type: "ip"};
		
		if(typeof nodes[link.target] !== "undefined")
		{
			nodes[link.target].ip = link.target;
			link.target = nodes[link.target];
		}
		else
			nodes[link.target] = {name: link.target, ip: link.target, type: "ip"};

		if(link.lq < 2)
			link.type = "good";
		else if(link.lq < 5)
			link.type = "ok";
		else
			link.type = "bad";
	});

	width = $(window).width();
	height = $(window).height();

	force = d3.layout.force()
		.nodes(d3.values(nodes))
		.links(links)
		.size([width, height])
		.linkDistance(100)
		.charge(-300)
		.on("tick", tick)
		.start();

	svg = d3.select("body").append("svg")
		.attr("width", width)
		.attr("height", height);

	// update drawing area, when window size changes
	$(window).resize(function() {
	  width = $(window).width();
	  height = $(window).height();
	  force.size([width, height]);
	  svg.attr("width", width)
		 .attr("height", height);
	});

	// Per-type markers, as they don't inherit styles.
	svg.append("defs").selectAll("marker")
		.data(["good", "ok", "bad"])
	  .enter().append("marker")
		.attr("id", function(d) { return d; })
		.attr("viewBox", "0 -5 10 10")
		.attr("refX", 15)
		.attr("refY", -1.5)
		.attr("markerWidth", 6)
		.attr("markerHeight", 6)
		.attr("orient", "auto")
	  .append("path")
		.attr("d", "M0,-5L10,0L0,5");

	path = svg.append("g").selectAll("path")
		.data(force.links())
	  .enter().append("path")
		.attr("class", function(d) { return "link " + d.type; })
		.attr("marker-end", function(d) { return "url(#" + d.type + ")"; });

	circle = svg.append("g").selectAll("circle")
		.data(force.nodes())
	  .enter().append("circle")
		.attr("r", 10)
		.call(force.drag);
	
	// add description link
	text = svg.append("g").selectAll("text")
		.data(force.nodes())
	  .enter().append("svg:a")
		.attr("xlink:href", function(d){ return "http://" +d.ip })
		.attr("target","_blank")

	text.append("svg:rect")
		.attr("x", 15)
		.attr("y",  function(d){
				if(d.type == "name")
					return "0.52em";
				else
					return "-0.33em";
			})
		.attr("fill", "black")
		.attr("opacity", 0.1)
		.attr("width", "4em")
		.attr("height", "0.6em");

	text.append("svg:text")
		.attr("x", 15)
		.attr("y", ".31em")
		.attr("fill", "black")
		.attr("class", function(d){ return d.type; })
		.text(function(d){ 
				if(d.type == "name") 
					return d.name; 
				else
					return "";
			});

	text.append("svg:text")
		.attr("x", 15)
		.attr("y",  function(d){
				if(d.type == "name")
					return "1.57em";
				else
					return "0.31em";
			})
		.attr("fill", "black")
		.attr("class", "ip")
		.text(function(d) { return d.ip; });
}

// Use elliptical arc path segments to doubly-encode directionality.
function tick() {
  path.attr("d", linkArc);
  circle.attr("transform", transform);
  text.attr("transform", transform);
}

function linkArc(d) {
  var dx = d.target.x - d.source.x,
      dy = d.target.y - d.source.y,
      dr = Math.sqrt(dx * dx + dy * dy);
  return "M" + d.source.x + "," + d.source.y + "A" + dr + "," + dr + " 0 0,1 " + d.target.x + "," + d.target.y;
}

function transform(d) {
  return "translate(" + d.x + "," + d.y + ")";
}

// ---------------------------------------------

$(init_network);
